use gilded_rose;

#[test]
pub fn test_normal_item() {
    let initial_sell_in = 5;
    let initial_quality = 10;
    let name = "NORMAL ITEM";

    let mut items = vec![gilded_rose::Item::new(name, initial_sell_in, initial_quality)];

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality);
    assert_eq!(item.sell_in, initial_sell_in);

    gilded_rose::update_quality(&mut items);

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality - 1);
    assert_eq!(item.sell_in, initial_sell_in - 1);
}

#[test]
pub fn test_normal_expired() {
    let initial_sell_in = 0;
    let initial_quality = 10;
    let name = "NORMAL ITEM";

    let mut items = vec![gilded_rose::Item::new(name, initial_sell_in, initial_quality)];

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality);
    assert_eq!(item.sell_in, initial_sell_in);

    gilded_rose::update_quality(&mut items);

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality - 2);
    assert_eq!(item.sell_in, initial_sell_in - 1);
}

#[test]
pub fn test_aged_brie() {
    let initial_sell_in = 5;
    let initial_quality = 10;
    let name = "Aged Brie";

    let mut items = vec![gilded_rose::Item::new(name, initial_sell_in, initial_quality)];

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality);
    assert_eq!(item.sell_in, initial_sell_in);

    gilded_rose::update_quality(&mut items);

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality + 1);
    assert_eq!(item.sell_in, initial_sell_in - 1);

    let initial_sell_in = 0;
    let initial_quality = 10;

    let mut items = vec![gilded_rose::Item::new(name, initial_sell_in, initial_quality)];

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality);
    assert_eq!(item.sell_in, initial_sell_in);

    gilded_rose::update_quality(&mut items);

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality + 2);
    assert_eq!(item.sell_in, initial_sell_in - 1);
}

#[test]
pub fn test_aged_brie_bounds() {
    // Near max Quality
    let initial_sell_in = 5;
    let initial_quality = 49;
    let name = "Aged Brie";

    let mut items = vec![gilded_rose::Item::new(name, initial_sell_in, initial_quality)];

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality);
    assert_eq!(item.sell_in, initial_sell_in);

    gilded_rose::update_quality(&mut items);

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, 50);
    assert_eq!(item.sell_in, initial_sell_in - 1);

    // Before Sellin Date
    let initial_sell_in = 5;
    let initial_quality = 50;

    let mut items = vec![gilded_rose::Item::new(name, initial_sell_in, initial_quality)];

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality);
    assert_eq!(item.sell_in, initial_sell_in);

    gilded_rose::update_quality(&mut items);

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, 50);
    assert_eq!(item.sell_in, initial_sell_in - 1);

    // On Sell in Date
    let initial_sell_in = 0;
    let initial_quality = 50;

    let mut items = vec![gilded_rose::Item::new(name, initial_sell_in, initial_quality)];

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality);
    assert_eq!(item.sell_in, initial_sell_in);

    gilded_rose::update_quality(&mut items);

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, 50);
    assert_eq!(item.sell_in, initial_sell_in - 1);

    // After Sell in Date
    let initial_sell_in = -10;
    let initial_quality = 50;

    let mut items = vec![gilded_rose::Item::new(name, initial_sell_in, initial_quality)];

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality);
    assert_eq!(item.sell_in, initial_sell_in);

    gilded_rose::update_quality(&mut items);

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, 50);
    assert_eq!(item.sell_in, initial_sell_in - 1);
}

#[test]
pub fn test_sulfuras() {
    let initial_sell_in = 10;
    let initial_quality = 80;
    let name = "Sulfuras, Hand of Ragnaros";

    let mut items = vec![gilded_rose::Item::new(name, initial_sell_in, initial_quality)];

    // Does not change before sell date
    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality);
    assert_eq!(item.sell_in, initial_sell_in);

    gilded_rose::update_quality(&mut items);

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality);
    assert_eq!(item.sell_in, initial_sell_in);

    let initial_sell_in = 0;
    let initial_quality = 80;

    let mut items = vec![gilded_rose::Item::new(name, initial_sell_in, initial_quality)];

    // Does not change on sell date
    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality);
    assert_eq!(item.sell_in, initial_sell_in);

    gilded_rose::update_quality(&mut items);

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality);
    assert_eq!(item.sell_in, initial_sell_in);

    // Does not change after sell date
    let initial_sell_in = -10;
    let initial_quality = 80;

    let mut items = vec![gilded_rose::Item::new(name, initial_sell_in, initial_quality)];

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality);
    assert_eq!(item.sell_in, initial_sell_in);

    gilded_rose::update_quality(&mut items);

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality);
    assert_eq!(item.sell_in, initial_sell_in);
}
#[test]
pub fn test_backstage_pass_normal() {
    // Long before Selling Date
    let initial_sell_in = 11;
    let initial_quality = 10;
    let name = "Backstage passes to a TAFKAL80ETC concert";

    let mut items = vec![gilded_rose::Item::new(name, initial_sell_in, initial_quality)];

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality);
    assert_eq!(item.sell_in, initial_sell_in);

    gilded_rose::update_quality(&mut items);

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality + 1);
    assert_eq!(item.sell_in, initial_sell_in - 1);
}

#[test]
pub fn test_backstage_pass_near() {
    // Long before Selling Date
    let initial_sell_in = 10;
    let initial_quality = 10;
    let name = "Backstage passes to a TAFKAL80ETC concert";

    let mut items = vec![gilded_rose::Item::new(name, initial_sell_in, initial_quality)];

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality);
    assert_eq!(item.sell_in, initial_sell_in);

    gilded_rose::update_quality(&mut items);

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality + 2);
    assert_eq!(item.sell_in, initial_sell_in - 1);
}

#[test]
pub fn test_backstage_pass_close() {
    // Long before Selling Date
    let initial_sell_in = 1;
    let initial_quality = 10;
    let name = "Backstage passes to a TAFKAL80ETC concert";

    let mut items = vec![gilded_rose::Item::new(name, initial_sell_in, initial_quality)];

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality);
    assert_eq!(item.sell_in, initial_sell_in);

    gilded_rose::update_quality(&mut items);

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality + 3);
    assert_eq!(item.sell_in, initial_sell_in - 1);
}

#[test]
pub fn test_backstage_pass_expired() {
    // Long before Selling Date
    let initial_sell_in = 0;
    let initial_quality = 10;
    let name = "Backstage passes to a TAFKAL80ETC concert";

    let mut items = vec![gilded_rose::Item::new(name, initial_sell_in, initial_quality)];

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality);
    assert_eq!(item.sell_in, initial_sell_in);

    gilded_rose::update_quality(&mut items);

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, 0);
    assert_eq!(item.sell_in, initial_sell_in - 1);
}

#[test]
#[ignore = "pending"]
pub fn test_conjured_item() {
    let initial_sell_in = 5;
    let initial_quality = 10;
    let name = "Conjured Mana Cake";

    let mut items = vec![gilded_rose::Item::new(name, initial_sell_in, initial_quality)];

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality);
    assert_eq!(item.sell_in, initial_sell_in);

    gilded_rose::update_quality(&mut items);

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality - 2);
    assert_eq!(item.sell_in, initial_sell_in - 1);
}

#[test]
#[ignore = "pending"]
pub fn test_conjured_item_expired() {
    let initial_sell_in = 0;
    let initial_quality = 10;
    let name = "Conjured Mana Cake";

    let mut items = vec![gilded_rose::Item::new(name, initial_sell_in, initial_quality)];

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality);
    assert_eq!(item.sell_in, initial_sell_in);

    gilded_rose::update_quality(&mut items);

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality - 4);
    assert_eq!(item.sell_in, initial_sell_in - 1);
}

#[test]
#[ignore = "pending"]
pub fn test_conjured_item_min() {
    let initial_sell_in = 0;
    let initial_quality = 3;
    let name = "Conjured Mana Cake";

    let mut items = vec![gilded_rose::Item::new(name, initial_sell_in, initial_quality)];

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality);
    assert_eq!(item.sell_in, initial_sell_in);

    gilded_rose::update_quality(&mut items);

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, 0);
    assert_eq!(item.sell_in, initial_sell_in - 1);

    let initial_sell_in = -10;
    let initial_quality = 3;
    let name = "Conjured Mana Cake";

    let mut items = vec![gilded_rose::Item::new(name, initial_sell_in, initial_quality)];

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, initial_quality);
    assert_eq!(item.sell_in, initial_sell_in);

    gilded_rose::update_quality(&mut items);

    let item = items.get(0).unwrap();
    assert_eq!(item.name, name);
    assert_eq!(item.quality, 0);
    assert_eq!(item.sell_in, initial_sell_in - 1);
}

#[test]
pub fn multiple_items() {
    let mut items = vec![
        gilded_rose::Item::new("NORMAL ITEM", 5, 10),
        gilded_rose::Item::new("Aged Brie", 3, 10)
    ];

    gilded_rose::update_quality(&mut items);


    let normal_item = items.get(0).unwrap();
    assert_eq!(normal_item.name, "NORMAL ITEM");
    assert_eq!(normal_item.quality, 9);
    assert_eq!(normal_item.sell_in, 4);

    let aged_brie = items.get(1).unwrap();
    assert_eq!(aged_brie.name, "Aged Brie");
    assert_eq!(aged_brie.quality, 11);
    assert_eq!(aged_brie.sell_in, 2);

}