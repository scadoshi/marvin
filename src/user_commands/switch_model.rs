use crate::{
    chat::{input::ChatInput, Chat},
    ui::horizontal_line,
};

pub trait SwitchModel {
    fn switch_model(&mut self) -> anyhow::Result<()>;
}

impl SwitchModel for Chat {
    fn switch_model(&mut self) -> anyhow::Result<()> {
        horizontal_line();
        self.clear_input();
        println!("Current model: {}", self.model());
        for (i, model) in self.model_options().iter().enumerate() {
            println!("{}. {}", i + 1, model.display_name);
        }
        horizontal_line();
        loop {
            println!("Select a model");
            horizontal_line();
            self.get_input();
            match self.input() {
                ChatInput::SendMessage(message) => {
                    let Some(selection) = self
                        .model_options()
                        .iter()
                        .enumerate()
                        .find(|(i, _)| {
                            message
                                .parse::<usize>()
                                .ok()
                                .and_then(|u| u.checked_sub(1))
                                .is_some_and(|u| u == *i)
                        })
                        .map(|(_, selection)| selection.to_owned())
                    else {
                        continue;
                    };
                    self.clear_input();
                    self.set_agent(selection)?;
                    println!("Model updated: {}", self.model());
                    break;
                }
                _ => break, // any other command should go to main loop for triaging
            }
        }
        Ok(())
    }
}
