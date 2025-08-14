use std::collections::{HashMap, VecDeque};
#[command(description = "warlord destroy: /destroy <username>")] Destroy(String),
#[command(description = "build by hand index: /build 0")] Build(usize),
#[command(description = "end your turn")] End,
#[command(description = "show your hand (DM)")] Hand,
#[command(description = "show status")] Status,
}


struct App { tables: RwLock<HashMap<ChatId, GameState>> }


impl App {
    fn new() -> Self { Self { tables: RwLock::new(HashMap::new()) } }
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let bot = Bot::from_env();
    info!("Bot started");
    let app = Arc::new(App::new());


    teloxide::commands_repl(bot, move |cx, cmd| handler(cx, cmd, app.clone()), Cmd::ty()).await;
}


async fn handler(bot: Bot, msg: Message, cmd: Cmd, app: Arc<App>) -> anyhow::Result<()> {
    let chat = msg.chat.id;
    let user = msg.from().unwrap();


    let mut tables = app.tables.write().await;
    let state = tables.entry(chat).or_insert_with(|| GameState::new(chat));


    let reply = match cmd {
        Cmd::New => { *state = GameState::new(chat); format!("New lobby created. Use /join then /start. Players 2-8.") }
        Cmd::Join => match state.join(user) { Ok(s)=>s, Err(e)=>format!("Cannot join: {}", e) }
        Cmd::Leave => match state.leave(user.id) { Ok(s)=>s, Err(e)=>format!("Cannot leave: {}", e) }
        Cmd::Start => match state.start() { Ok(s)=>s, Err(e)=>format!("Cannot start: {}", e) }
        Cmd::Pickrole(arg) => match parse_role(&arg) { Some(r)=> match state.pick_role(user.id, r) { Ok(s)=>s, Err(e)=>format!("{}", e) }, None=>"Unknown role".into() }
        Cmd::Income(x) => state.take_income_or_draw(user.id, x.to_lowercase().starts_with('g')).unwrap_or_else(|e| format!("{}", e))
            Cmd::Power(args) => {
                // special: thief auto-collect when act
                let pre = if state.find_player(user.id).and_then(|p| p.role) == Some(Role::Thief) {
                    if let Some(amt) = state.collect_robbed_gold(user.id) { format!("Thief collects {} gold from victim. ", amt) } else { String::new() }
                } else { String::new() };
                format!("{}{}", pre, state.power(user.id, &args).unwrap_or_else(|e| format!("{}", e)))
            }
        Cmd::Destroy(name) => state.warlord_destroy(user.id, &name).unwrap_or_else(|e| format!("{}", e))
            Cmd::Build(i) => state.build(user.id, i).unwrap_or_else(|e| format!("{}", e))
            Cmd::End => state.end_turn_and_maybe_advance_round().unwrap_or_else(|| "Advanced.".into()),
        Cmd::Hand => {
            if let Some(p) = state.find_player(user.id) {
                let cards = p.hand.iter().enumerate().map(|(i,c)| format!("{}: {}({})", i, c.name, c.cost)).collect::<Vec<_>>().join(", ");
                format!("Your hand: {}\nGold: {}\nBuilt: {}", cards, p.gold, p.built.iter().map(|c| c.name).collect::<Vec<_>>().join(", "))
            } else { "Not seated.".into() }
        }
        Cmd::Status => {
            match &state.phase {
                GamePhase::Lobby => format!("Lobby: {} players: {}", state.players.len(), state.players.iter().map(|p| p.username.clone()).collect::<Vec<_>>().join(", ")),
                GamePhase::Draft { draft_order, available_roles } => {
                    let cur = state.current_drafter().and_then(|id| state.find_player(id)).map(|p| p.username.clone()).unwrap_or("?".into());
                    format!("Round {} Draft. Next: {}. Available: {}", state.round, cur, available_roles.iter().map(|r| format_role(*r)).collect::<Vec<_>>().join(", "))
                }
                GamePhase::Turns { current_role } => {
                    let intro = state.start_turn_text();
                    format!("Round {} | Role {}. {}", state.round, current_role, intro)
                }
                GamePhase::Finished => {
                    let scores = state.compute_scores();
                    format!("Finished.\n{}", format_scores(&state.players, &scores))
                }
            }
        }
    };


    bot.send_message(chat, reply).await?;


    Ok(())
}
