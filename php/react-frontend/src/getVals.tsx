import React from 'react';
import { GetData } from './api';

export class FetchAndDisplay extends React.Component {
  constructor(props: object) {
    super(props);
    this.state = { data: [] };
  }

  get() {
    // @ts-ignore
    GetData(this.props.path).then((data) => {
      console.log(data);
      this.setState({ data: data });
    });
  }

  render() {
    // @ts-ignore
    const listElems = this.state.data.map((v) => <li>{JSON.stringify(v)}</li>);
    return (
      <div>
        <ul>{listElems}</ul>
        <button
          onClick={() => {
            this.get();
          }}
        >
          Get Values
        </button>
      </div>
    );
  }
}
