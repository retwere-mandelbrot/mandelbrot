import './App.css'
import Map from './Map'
import { LatLngExpression } from 'leaflet'

function App() {
  const pos: LatLngExpression = [51.505, -0.09]
  return (
    <div className="App">
      <Map position={pos} height={window.innerHeight}></Map>
    </div>
  )
}

export default App
