import Map from './Map'

function App() {
  return (
    <div className="App">
      <Map height={window.innerHeight} url="http://localhost:8000/mandelbrot" />
    </div>
  )
}

export default App
