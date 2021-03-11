use crate::connection::conn::get_pool;
use crate::entities::entity_utils::CRUD;
use crate::entities::{
    caretaker::{Caretaker, NewCaretaker},
    cat::{Cat, NewCat},
    city::{City, NewCity},
    dog::{Dog, NewDog},
    pet::{NewPet, Pet},
    vet::{NewVet, Vet},
};
use crate::misc::redis_interactions::create_restricted_user_key_pair;

// This script swaps the user to a user with restricted privileges
pub async fn execute_restricted_user_creation() -> anyhow::Result<()> {
    let pool = get_pool().await?;
    create_restricted_user_key_pair().expect("Could not create restricted user key value pair");
    sqlx::query_file!("src/misc/usr.sql").execute(&pool).await?;

    Ok(())
}

// Entity based pop
pub async fn alt_pop() -> anyhow::Result<()> {
    let pool = get_pool()
        .await
        .expect("Pool could not be created in city_handler read");

    for i in 0..2 {
        let city = NewCity::new(
            String::from(format!("city_{}_zip_code", i)),
            String::from(format!("city_{}_name", i)),
        );
        City::create(&pool, city).await?;
    }

    for i in 0..10 {
        let caretaker = NewCaretaker::new(
            String::from(format!("caretaker_{}_name", i)),
            String::from(format!("caretaker_{}_phone", i)),
            String::from(format!("caretaker_{}_street", i)),
            (i % 2) + 1,
        );
        Caretaker::create(&pool, caretaker).await?;
    }

    for i in 0..2 {
        let vet = NewVet::new(
            String::from(format!("vet_{}_cvr", i)),
            String::from(format!("vet_{}_phone", i)),
            String::from(format!("vet_{}_street", i)),
            (i % 2) + 1,
        );

        Vet::create(&pool, vet).await?;
    }

    for i in 0..10 {
        let pet = NewPet::new(String::from(format!("pet_{}_name", i)), i, (i % 2) + 1);
        Pet::create(&pool, pet).await?;
    }

    let fur_colors = vec!["brown", "black", "white"];

    for i  in 0..10 {
        let cat = NewCat::new(
            String::from(format!("cat_{}_name",i)),
            i,
            ((i%2) +1) as i32,
            fur_colors[i as usize%3].to_owned()
            
        );
        Cat::create(&pool, cat).await?;
    }

    let bark_pitches = vec![12.2, 3.9, 132.2];

    for i in 0..5 {
        let dog = NewDog::new(
            String::from(format!("dog_{}_name",i)),
            i,
            ((i%2)+1) as i32,
            bark_pitches[i as usize%3].to_owned()
        );
        Dog::create(&pool, dog).await?;
    }
    Ok(())
}
