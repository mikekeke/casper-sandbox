import axios from 'axios';

function Query(props) {

    return (
      <button onClick={() => queryMessage()}>Query Message</button>
    );
  }

  function queryMessage() {
    axios.get('http://localhost:3001/query_msg').then(response => {
        alert(response.data);
    })
}

export default Query;