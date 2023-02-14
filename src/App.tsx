import './App.css'
import Map from './Map'
import { LatLngExpression } from 'leaflet'

function App() {
  return (
    <div className="App">
      <Map height={window.innerHeight}></Map>
    </div>
  )
}

export default App
