use yew::prelude::*;

pub mod row;
use row::Row;

use self::row::RowProps;

pub enum Msg {
    AppendSerialEvent(usize),
}

/// The `Parent` component holds some state that is updated when its children are clicked
pub struct Graphcet {
    rows: Vec<RowProps>,
    uid_counter: usize,
}
impl Graphcet {
    fn get_and_increment_uid(&mut self) -> usize {
        let uid = self.uid_counter;
        self.uid_counter += 1;
        uid
    }
}

impl Component for Graphcet {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            rows: vec![
                RowProps {
                    uid: 0,
                    index: 0,
                    on_clicked: _ctx.link().callback(move |_| Msg::AppendSerialEvent(0)),
                    action_name: "HorizontalCylPaP := 1".to_string(),
                    transitions: "X0".to_string(),
                },
                RowProps {
                    uid: 1,
                    index: 1,
                    on_clicked: _ctx.link().callback(move |_| Msg::AppendSerialEvent(1)),
                    action_name: "VerticalCylPaP := 1".to_string(),
                    transitions: "X1".to_string(),
                },
            ],
            uid_counter: 2,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AppendSerialEvent(index) => {
                let new_index = index + 1;

                let new_row = RowProps {
                    uid: self.get_and_increment_uid(),
                    index: new_index,
                    on_clicked: _ctx
                        .link()
                        .callback(move |_| Msg::AppendSerialEvent(new_index)),
                    action_name: "".to_string(),
                    transitions: "".to_string(),
                };

                if new_index + 1 > self.rows.len() {
                    self.rows.push(new_row);
                } else {
                    self.rows.insert(new_index, new_row);
                }

                for i in (new_index + 1)..self.rows.len() {
                    self.rows[i].index = i;
                    self.rows[i].on_clicked =
                        _ctx.link().callback(move |_| Msg::AppendSerialEvent(i));
                }

                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="parent">
                {for self.rows.iter().map(|row| {
                    html! {
                        <Row key={row.uid} uid={row.uid} index={row.index} on_clicked={row.on_clicked.clone()} action_name={row.action_name.clone()} transitions={row.transitions.clone()} />
                    }
                })}
            </div>
        }
    }
}
