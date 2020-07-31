import React from 'react';
import logo from './logo.svg';
import './App.css';

class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = {camera:  false}
    this.handleSubmit = this.handleSubmit.bind(this);
  }

  handleSubmit(event) {
	  this.setState({camera: true})
	  fetch("/" +this.state.camera)
  .catch(function (error) {
    console.log("Error: " + error);
  });
    event.preventDefault();
  }

  render() {
    return (
      <form onSubmit={this.handleSubmit}>
        <input type="submit" value="Submit" />
      </form>
    );
  }
}

export default App;
