import { ComponentMeta, ComponentStory } from '@storybook/react'

import { Icon } from 'leaflet'
import { Marker, Popup } from 'react-leaflet'
import Map from './Map'

import MarkerImg from './marker.png'

export default {
  title: 'Map',
  component: Map,
  parameters: {
    layout: 'fullscreen'
  }
} as ComponentMeta<typeof Map>

const Template: ComponentStory<typeof Map> = args => <Map {...args} />

export const Scale = Template.bind({})
const markerIcon = new Icon({ iconUrl: MarkerImg, iconAnchor: [25, 82] })
Scale.args = {
  height: 800,
  url: 'http://localhost:8000/mandelbrot',
  children: [
    <Marker position={[0.0, 0.0]} icon={markerIcon}>
      <Popup>
        <b>0.0, 0.0</b>
      </Popup>
    </Marker>,
    <Marker position={[-4.0, -4.0]} icon={markerIcon}>
      <Popup>
        <b>-4.0, -4.0</b>
      </Popup>
    </Marker>,
    <Marker position={[4.0, 4.0]} icon={markerIcon}>
      <Popup>
        <b>4.0, 4.0</b>
      </Popup>
    </Marker>,
    <Marker position={[-1.25, -2.0]} icon={markerIcon}>
      <Popup>
        <b>-2.0, -1.25</b>
      </Popup>
    </Marker>,
    <Marker position={[1.25, 0.5]} icon={markerIcon}>
      <Popup>
        <b>0.5, 1.25</b>
      </Popup>
    </Marker>,
    <Marker position={[0.6, -0.4]} icon={markerIcon}>
      <Popup>
        <b>-0.4, 0.6</b>
      </Popup>
    </Marker>
  ]
}

export const Julia = Template.bind({})
Julia.args = {
  height: 800,
  url: 'http://localhost:8000/julia/-0.4/0.6'
}

export const Mandelbrot = Template.bind({})
Mandelbrot.args = {
  height: 800,
  url: 'http://localhost:8000/mandelbrot'
}
