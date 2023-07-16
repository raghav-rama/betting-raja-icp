import * as React from "react";
import { render } from "react-dom";
import { football_betting_backend } from "../../declarations/football_betting_backend";

const App = () => {
  const [name, setName] = React.useState("");
  const [message, setMessage] = React.useState("");

  async function doGreet() {
    const greeting = await football_betting_backend.greet_me(name);
    setMessage(greeting);
  }

  async function doGreet2() {
    const greeting = await football_betting_backend.get_available_games();
    setMessage(greeting);
  }

  async function doGreet3() {
    const greeting = await football_betting_backend.send_bet();
    setMessage(greeting);
  }

  return (
    <>
      <div style={{ fontSize: "30px" }}>
        <div style={{ backgroundColor: "pink" }}>
          <p>Greetings from Betting Raja!</p>
          <p> Type your name and click the button</p>
        </div>
        <div style={{ margin: "30px" }}>
          <input
            id="name"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
          <button onClick={doGreet}>Greet</button>
          <button onClick={doGreet2}>Greet2</button>
          <button onClick={doGreet3}>Greet3</button>
          <div>
            <p>Message from backend:</p>
            <p>{message}</p>
          </div>
        </div>
      </div>
    </>
  );
};

render(<App />, document.getElementById("app"));
