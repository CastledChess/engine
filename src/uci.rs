use crate::search::{Search, SearchEngine, TimeControl};
use queues::{queue, IsQueue, Queue};
use shakmaty::fen::Fen;
use shakmaty::uci::UciMove;
use shakmaty::{CastlingMode, Chess, Position};

pub trait UciParser {
    fn parse_command(&mut self, command: &str);
    fn parse_tokens(&mut self, tokens: &mut Queue<&str>);
    fn handle_go(&mut self, tokens: &mut Queue<&str>);
    fn handle_btime(&mut self, tokens: &mut Queue<&str>);
    fn handle_wtime(&mut self, tokens: &mut Queue<&str>);
    fn handle_go_depth(&mut self, tokens: &mut Queue<&str>);
    fn handle_go_movetime(&mut self, tokens: &mut Queue<&str>);
    fn handle_position(&mut self, tokens: &mut Queue<&str>);
    fn handle_position_startpos(&mut self, tokens: &mut Queue<&str>);
    fn handle_position_fen(&mut self, tokens: &mut Queue<&str>);
    fn handle_setoption(&self, tokens: &mut Queue<&str>);
    fn handle_go_infinite(&mut self, tokens: &mut Queue<&str>);
    fn handle_ucinewgame(&self);
    fn handle_isready(&self);
    fn handle_quit(&self);
    fn handle_uci(&self);
}

pub struct Uci {
    pub search: Search,
}

impl UciParser for Uci {
    fn parse_command(&mut self, command: &str) {
        let tokens_vec: Vec<&str> = command.split_whitespace().collect();
        let mut tokens: Queue<&str> = queue![];

        for token in tokens_vec {
            tokens.add(token).unwrap();
        }

        self.parse_tokens(&mut tokens);
    }

    fn parse_tokens(&mut self, tokens: &mut Queue<&str>) {
        let first_token = tokens.remove().unwrap();

        match first_token {
            "uci" => self.handle_uci(),
            "isready" => self.handle_isready(),
            "quit" => self.handle_quit(),
            "setoption" => self.handle_setoption(tokens),
            "ucinewgame" => self.handle_ucinewgame(),
            "position" => self.handle_position(tokens),
            "go" => self.handle_go(tokens),
            _ => println!("Unknown command: {}", first_token),
        }
    }

    fn handle_go(&mut self, tokens: &mut Queue<&str>) {
        let token = tokens.remove();

        match token.is_ok() {
            true => match token.unwrap() {
                "btime" => self.handle_btime(tokens),
                "wtime" => self.handle_wtime(tokens),
                "depth" => self.handle_go_depth(tokens),
                "movetime" => self.handle_go_movetime(tokens),
                "infinite" => self.handle_go_infinite(tokens),
                _ => println!("Unknown go command: {}", token.unwrap()),
            },

            false => {
                self.search.go();
            }
        }
    }

    fn handle_btime(&mut self, tokens: &mut Queue<&str>) {
        let token = tokens.remove().unwrap();
        let time = token.parse::<u32>().unwrap();

        self.search.depth = 1000;
        self.search.time_control = TimeControl::WOrBTime;
        self.search.btime = time;

        self.handle_go(tokens);
    }

    fn handle_wtime(&mut self, tokens: &mut Queue<&str>) {
        let token = tokens.remove().unwrap();
        let time = token.parse::<u32>().unwrap();

        self.search.depth = 1000;
        self.search.time_control = TimeControl::WOrBTime;
        self.search.wtime = time;

        self.handle_go(tokens);
    }

    fn handle_go_depth(&mut self, tokens: &mut Queue<&str>) {
        let token = tokens.remove().unwrap();
        let depth = token.parse::<u64>().unwrap();

        self.search.depth = depth as u32;
        self.search.time_control = TimeControl::None;

        self.handle_go(tokens);
    }

    fn handle_go_movetime(&mut self, tokens: &mut Queue<&str>) {
        let token = tokens.remove().unwrap();
        let time = token.parse::<u32>().unwrap();

        self.search.movetime = time;
        self.search.time_control = TimeControl::MoveTime;
        self.search.depth = 1000;

        self.handle_go(tokens);
    }

    fn handle_position(&mut self, tokens: &mut Queue<&str>) {
        let token = tokens.remove().unwrap();

        match token {
            "startpos" => {
                let _ = self.handle_position_startpos(tokens);
            }
            "fen" => self.handle_position_fen(tokens),
            _ => println!("Unknown position command: {}", token),
        }
    }

    fn handle_position_startpos(&mut self, tokens: &mut Queue<&str>) {
        self.search.game = Chess::default();

        if let Some(moves) = tokens.remove().ok() {
            if moves != "moves" {
                return;
            }

            while let Some(move_str) = tokens.remove().ok() {
                let uci_move = move_str.parse::<UciMove>().ok();
                let game = self.search.game.clone();
                let legal = uci_move.unwrap().to_move(&game).ok().unwrap();
                game.play(&legal).unwrap();
            }
        }
    }

    fn handle_position_fen(&mut self, tokens: &mut Queue<&str>) {
        let fen: Fen = tokens.remove().ok().unwrap().parse().unwrap();

        self.search.game = fen.clone().into_position(CastlingMode::Standard).unwrap();

        if let Some(moves) = tokens.remove().ok() {
            if moves != "moves" {
                return;
            }

            while let Some(move_str) = tokens.remove().ok() {
                if let uci_move = move_str.parse::<UciMove>().ok() {
                    let game = self.search.game.clone();
                    let legal = uci_move.unwrap().to_move(&game).ok().unwrap();
                    game.play(&legal).unwrap();
                }
            }
        }
    }

    fn handle_setoption(&self, tokens: &mut Queue<&str>) {
        tokens.remove().unwrap(); // name
        let name = tokens.remove().unwrap();
        tokens.remove().unwrap(); // value
        let value = tokens.remove().unwrap();

        if name.is_empty() || value.is_empty() {
            return;
        }

        match name {
            "MoveOverhead" => println!("info string set move overhead"), // search.move_overhead = value.parse();
            _ => println!("info string unknown option: {}", name),
        }
    }

    fn handle_go_infinite(&mut self, tokens: &mut Queue<&str>) {
        // search.max_depth = 1000;
        // search.time_control = infinite;

        self.handle_go(tokens);
    }

    fn handle_ucinewgame(&self) {
        // search.position = startpos;
        // search.clear();
    }

    fn handle_isready(&self) {
        // wait for engine to be ready

        println!("readyok");
    }

    fn handle_quit(&self) {
        std::process::exit(0);
    }

    fn handle_uci(&self) {
        println!("id name CastledEngine");
        println!("id author CastledChess");

        // TODO: print all options
        // for option in vec![
        //     "MoveOverhead",
        //     "UCI_Chess960",
        //     "UCI_AnalyseMode",
        //     "UCI_LimitStrength",
        //     "UCI_Elo",
        //     "UCI_ShowWDL",
        //     "UCI_ShowCurrLine",
        //     "UCI_ShowRefutations
        // "].iter() {
        //     println!("option name {} type spin default 0", option);
        // }
    }
}
