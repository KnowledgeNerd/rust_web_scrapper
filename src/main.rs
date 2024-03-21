//  Code is from https://www.zenrows.com/blog/rust-web-scraping
//  All right reserved by them.  I am posting this to my GitHub for further use
//  by myself.

struct PokemonProduct {
    url: Option<String>,
    image: Option<String>,
    name: Option<String>,
    price: Option<String>,
}
fn main() {
    let response = reqwest::blocking::get("https://scrapeme.live/shop/");
    let html_content = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&html_content);

    let html_product_selector = scraper::Selector::parse("li.product").unwrap();
    let html_products = document.select(&html_product_selector);

    let mut pokemon_products: Vec<PokemonProduct> = Vec::new();

    for html_product in html_products {
        // scraping logic to retrieve the info
        // of interest
        let url = html_product
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned);
        let image = html_product
            .select(&scraper::Selector::parse("img").unwrap())
            .next()
            .and_then(|img| img.value().attr("src"))
            .map(str::to_owned);
        let name = html_product
            .select(&scraper::Selector::parse("h2").unwrap())
            .next()
            .map(|h2| h2.text().collect::<String>());
        let price = html_product
            .select(&scraper::Selector::parse(".price").unwrap())
            .next()
            .map(|price| price.text().collect::<String>());

        // instantiate a new Pokemon product
        // with the scraped data and add it to the list
        let pokemon_product = PokemonProduct {
            url,
            image,
            name,
            price,
        };
        pokemon_products.push(pokemon_product);
    }

    // create the CSV output file
    let path = std::path::Path::new("products.csv");
    let mut writer = csv::Writer::from_path(path).unwrap();

    // append the header to the CSV
    writer
        .write_record(&["url", "image", "name", "price"])
        .unwrap();

    // populate the output file
    for product in pokemon_products {
        let url = product.url.unwrap();
        let image = product.image.unwrap();
        let name = product.name.unwrap();
        let price = product.price.unwrap();
        writer.write_record(&[url, image, name, price]).unwrap();
    }

    // free up the resources
    writer.flush().unwrap();
}

//   ---------------------------------------------
//  | Some Details on How the Web Scrapping Works |
//   ---------------------------------------------
//
//   These commands can be run in the Dev Tool's Console
//   to emulate how the scraper is performing the scrapping.
//   
//      -- How to select the product listing items --
//  Rust -->
//    // Collects All Products at Once
//    let html_product_selector = scraper::Selector::parse("li.product").unwrap();
//    let html_products = document.select(&html_product_selector);
//
//  JS -->
//    // One Product at a Time
//    var firstproduct = document.querySelector("li.product")
//    var nextproduct = document.querySelector("li.product").nextElementSibling
//
//      -- How to query the details from each product --
//  Rust -->
//    let price = html_product
//    .select(&scraper::Selector::parse(".price").unwrap())
//    .next()
//    .map(|price| price.text().collect::<String>());
//
//  JS -->
//    var price = document.querySelector("li.product").querySelector(".price").textContent
//
//
// The following can quickly show the details of the first product
// ---------------------------------------------------------------
// var price = document.querySelector("li.product").querySelector(".price").textContent
// var link = document.querySelector("li.product").querySelector("a").attributes.href.textContent
// var image = document.querySelector("li.product").querySelector("img").attributes.src.textContent
// var name = document.querySelector("li.product").querySelector("h2").textContent
// console.log(name, " ", image, " ", link, " ", price)