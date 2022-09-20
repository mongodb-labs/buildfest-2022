
console.log('hi!')

const stream = document.getElementById('mta-stream')

console.log(stream);

const dateFmt = Intl.DateTimeFormat('en-US', { dateStyle: 'full', timeStyle: 'long' });

let occurrences = 0;
const interval = setInterval(() => {
    stream.setState({
        items: [...stream.state.items, { created: dateFmt.format(new Date()), ...Array.from({length: 3}, () => Math.random()) }]
    })

    if (occurrences === 10) {
        clearInterval(interval)
    }
}, 3000)
