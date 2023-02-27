import { ComponentMeta, ComponentStory } from '@storybook/react'
import { MapContainer } from 'react-leaflet'
import GridLayer from './GridLayer'
import { FRACTAL_CRS } from './Map'

export default {
  title: 'GridLayer',
  component: GridLayer,
  parameters: {
    layout: 'fullscreen'
  }
} as ComponentMeta<typeof GridLayer>

const Template: ComponentStory<typeof GridLayer> = args => {
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
      <GridLayer {...args} />
    </MapContainer>
  )
}

export const Grid = Template.bind({})
Grid.args = {
  createTile: function (coords) {
    var tile = document.createElement('div')
    tile.innerHTML = [coords.x, coords.y, coords.z].join(', ')
    tile.style.outline = '1px solid red'
    return tile
  }
}

export const Timer = Template.bind({})
Timer.args = {
  createTile: function (coords) {
    var tile = document.createElement('div')
    tile.innerHTML = [coords.x, coords.y, coords.z].join(', ')
    tile.style.outline = '1px solid red'

    setTimeout(function () {
      tile.style.outline = '1px solid green'
    }, 500 + Math.random() * 1500)

    return tile
  }
}

export const TimerCallback = Template.bind({})
TimerCallback.args = {
  createTile: function (coords, done) {
    var tile = document.createElement('div')
    tile.innerHTML = [coords.x, coords.y, coords.z].join(', ')
    tile.style.outline = '1px solid red'

    setTimeout(function () {
      tile.style.outline = '1px solid green'
      done(undefined, tile) // Syntax is 'done(error, tile)'
    }, 500 + Math.random() * 1500)

    return tile
  }
}
