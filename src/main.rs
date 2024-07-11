use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Address {
    cep: String,
    logradouro: String,
    complemento: Option<String>,
    bairro: String,
    localidade: String,
    uf: String,
    unidade: Option<String>,
    ibge: Option<String>,
    gia: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cep = "01001000"; // CEP de exemplo

    let url = format!("https://viacep.com.br/ws/{}/json/", cep);
    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let address: Address = response.json().await?;
        println!("CEP: {}", address.cep);
        println!("Logradouro: {}", address.logradouro);
        println!("Complemento: {}", address.complemento.unwrap_or_default());
        println!("Bairro: {}", address.bairro);
        println!("Localidade: {}", address.localidade);
        println!("UF: {}", address.uf);
        println!("Unidade: {:?}", address.unidade);
        println!("IBGE: {:?}", address.ibge);
        println!("GIA: {:?}", address.gia);
    } else {
        println!("Erro ao buscar o CEP: {}", response.status());
    }

    Ok(())
}
