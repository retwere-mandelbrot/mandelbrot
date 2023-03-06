import { ComponentMeta, ComponentStory } from '@storybook/react'
import { MapContainer } from 'react-leaflet'
import FractalLayer from './FractalLayer'
import { FRACTAL_CRS } from './Map'

export default {
  title: 'FractalLayer',
  component: FractalLayer,
  parameters: {
    layout: 'fullscreen'
  }
} as ComponentMeta<typeof FractalLayer>

const Template: ComponentStory<typeof FractalLayer> = args => {
return (
    <MapContainer
      center={[0.0, 0.0]}
      zoom={3}
      minZoom={0}
      maxZoom={50}
      scrollWheelZoom={false}
      style={{ height: 800 }}
      crs={FRACTAL_CRS}
    >
      <FractalLayer />
    </MapContainer>
  )
}

export const Fractal = Template.bind({})
Fractal.args = {}
