use super::{Field, Gamerecord, Step};
use crate::models;

pub trait IntoDb<D> {
    fn into_db(self) -> D;
}

pub trait FromDb<D> where Self: Sized {
    type Err;

    fn from_db(db_entity: D) -> Result<Self, Self::Err>;
}

impl FromDb<models::Gamerecord> for Gamerecord {
    type Err = ();

    fn from_db(db_entity: models::Gamerecord) -> Result<Gamerecord, ()> {
        if db_entity.rule.len() != 9 { return Err(()); }

        let mut rule = [[false; 3]; 3];

        for i in 0..9 {
            rule[i / 3][i % 3] = db_entity.rule[i];
        }

        let steps = db_entity.steps.iter()
            .map(|s| Step::from_db(s))
            .collect::<Result<_, _>>()?;

        let initial_field = Field::from_db(&db_entity)?;

        let gamerecord = Gamerecord {
            id: db_entity.id.to_string(),
            player_name: db_entity.player_name,
            start_time: db_entity.start_time,
            initial_field,
            rule,
            steps
        };

        Ok(gamerecord)
    }
}

impl FromDb<&models::Gamerecord> for Field {
    type Err = ();

    fn from_db(db_entity: &models::Gamerecord) -> Result<Field, ()> {
        let models::Gamerecord {
            initial_field_width: width,
            initial_field_height: height,
            initial_field: cells,
            ..
        } = db_entity;

        let cells = cells.chunks(*width as usize)
            .map(|c| c.iter().map(|&b| b).collect())
            .collect();

        let field = Field {
            width: *width as i32,
            height: *height as i32,
            cells
        };

        Ok(field)
    }
}

impl IntoDb<String> for Step {
    fn into_db(self) -> String {
        format!("{},{},{}", self.time, self.point.0, self.point.1)
    }
}

impl FromDb<&str> for Step {
    type Err = ();

    fn from_db(db_entity: &str) -> Result<Step, ()> {
        match db_entity.split(',').collect::<Vec<_>>().as_slice() {
            [time_str, x_str, y_str] => {
                let step = Step {
                    time: time_str.parse::<u64>().map_err(|_| ())?,
                    point: (
                        x_str.parse::<i32>().map_err(|_| ())?,
                        y_str.parse::<i32>().map_err(|_| ())?,
                    )
                };

                Ok(step)
            },

            _ => Err(())
        }
    }
}
