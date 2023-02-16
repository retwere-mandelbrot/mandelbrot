import { ComponentMeta, ComponentStory } from '@storybook/react'

import Map from './Map'

export default {
  title: 'Map',
  component: Map,
  parameters: {
    layout: 'fullscreen'
  }
} as ComponentMeta<typeof Map>

const Template: ComponentStory<typeof Map> = args => <Map {...args} />

export const Display = Template.bind({})
Display.args = {
  height: 800
}
