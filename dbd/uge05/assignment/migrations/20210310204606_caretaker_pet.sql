-- Add migration script here

CREATE TABLE IF NOT EXISTS caretaker_pet (
    pet_id INTEGER REFERENCES pet(id) ON UPDATE CASCADE,
    caretaker_id INTEGER REFERENCES caretaker(id) ON UPDATE CASCADE 
)