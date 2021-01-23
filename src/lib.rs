#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: &str, sell_in: i32, quality: i32) -> Self {
        Self {
            name: name.into(),
            sell_in,
            quality,
        }
    }
}

pub fn update_quality(items: &mut Vec<Item>) {
    for item in items.iter_mut() {
        if item.name != "Aged Brie" && item.name != "Backstage passes to a TAFKAL80ETC concert" {
            if item.quality > 0 {
                if item.name != "Sulfuras, Hand of Ragnaros" {
                    item.quality -= 1;
                }
            }
        } else {
            if item.quality < 50 {
                item.quality += 1;
                if item.name == "Backstage passes to a TAFKAL80ETC concert" {
                    if item.sell_in < 11 {
                        if item.quality < 50 {
                            item.quality += 1
                        }
                    }
                    if item.sell_in < 6 {
                        if item.quality < 50 {
                            item.quality += 1
                        }
                    }
                }
            }
        }

        if item.name != "Sulfuras, Hand of Ragnaros" {
            item.sell_in -= 1;
        }

        if item.sell_in < 0 {
            if item.name != "Aged Brie" {
                if item.name != "Backstage passes to a TAFKAL80ETC concert" {
                    if item.quality > 0 {
                        if item.name != "Sulfuras, Hand of Ragnaros" {
                            item.quality -= 1;
                        }
                    }
                } else {
                    item.quality = item.quality - item.quality;
                }
            } else {
                if item.quality < 50 {
                    item.quality += 1
                }
            }
        }
    }
}
