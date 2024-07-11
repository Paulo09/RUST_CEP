Para criar uma aplicação simples em React que consome uma API de CEP, vamos usar o Create React App para configurar o projeto rapidamente. Utilizaremos a API pública "ViaCEP" para obter os dados de um CEP.

Passo a Passo
Criar o Projeto

Primeiro, vamos criar um novo projeto React usando Create React App:

bash
Copiar código
npx create-react-app cep-consumer
cd cep-consumer
Instalar Axios

Vamos usar Axios para fazer as requisições HTTP. Instale o Axios:

bash
Copiar código
npm install axios
Modificar o Componente App

Abra o arquivo src/App.js e modifique-o para incluir o código que fará a busca do CEP e exibirá os dados:

jsx
Copiar código
import React, { useState } from 'react';
import axios from 'axios';
import './App.css';

function App() {
  const [cep, setCep] = useState('');
  const [address, setAddress] = useState(null);
  const [error, setError] = useState('');

  const handleInputChange = (event) => {
    setCep(event.target.value);
  };

  const handleSearch = async () => {
    try {
      setError('');
      const response = await axios.get(`https://viacep.com.br/ws/${cep}/json/`);
      if (response.data.erro) {
        setError('CEP não encontrado.');
        setAddress(null);
      } else {
        setAddress(response.data);
      }
    } catch (error) {
      setError('Erro ao buscar o CEP.');
      setAddress(null);
    }
  };

  return (
    <div className="App">
      <header className="App-header">
        <h1>Busca de CEP</h1>
        <div>
          <input
            type="text"
            value={cep}
            onChange={handleInputChange}
            placeholder="Digite o CEP"
          />
          <button onClick={handleSearch}>Buscar</button>
        </div>
        {error && <p className="error">{error}</p>}
        {address && (
          <div className="address">
            <p><strong>CEP:</strong> {address.cep}</p>
            <p><strong>Logradouro:</strong> {address.logradouro}</p>
            <p><strong>Complemento:</strong> {address.complemento}</p>
            <p><strong>Bairro:</strong> {address.bairro}</p>
            <p><strong>Localidade:</strong> {address.localidade}</p>
            <p><strong>UF:</strong> {address.uf}</p>
            <p><strong>IBGE:</strong> {address.ibge}</p>
            <p><strong>GIA:</strong> {address.gia}</p>
            <p><strong>DDD:</strong> {address.ddd}</p>
            <p><strong>SIAFI:</strong> {address.siafi}</p>
          </div>
        )}
      </header>
    </div>
  );
}

export default App;
Adicionar Estilos

Vamos adicionar alguns estilos simples para a aplicação. Crie o arquivo src/App.css (se não existir) e adicione o seguinte:

css
Copiar código
.App {
  text-align: center;
}

.App-header {
  background-color: #282c34;
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  font-size: calc(10px + 2vmin);
  color: white;
}

.App-header input {
  padding: 10px;
  font-size: 16px;
}

.App-header button {
  padding: 10px 20px;
  font-size: 16px;
  margin-left: 10px;
  cursor: pointer;
}

.error {
  color: red;
  margin-top: 20px;
}

.address {
  text-align: left;
  margin-top: 20px;
}

.address p {
  margin: 5px 0;
}
Executar a Aplicação

Para iniciar a aplicação, execute o seguinte comando:

bash
Copiar código
npm start
Explicação do Código
Estados: Usamos o useState para gerenciar os estados do CEP (cep), do endereço (address) e das mensagens de erro (error).
Entrada de CEP: Um campo de input para o usuário digitar o CEP e um botão para buscar o CEP.
Busca de CEP: Ao clicar no botão, a função handleSearch é chamada, que faz a requisição à API ViaCEP usando Axios e atualiza o estado com os dados do endereço ou uma mensagem de erro.
Exibição de Dados: Se houver um erro, ele é exibido. Se houver um endereço, ele é exibido com os detalhes do CEP.
Agora você tem uma aplicação React simples que consome uma API de CEP e exibe os dados de um endereço.
