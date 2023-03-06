import { ComponentMeta, ComponentStory } from '@storybook/react'
import Mandelbrot from './Mandelbrot'

export default {
  title: 'Mandelbrot',
  component: Mandelbrot,
  parameters: {
    layout: 'fullscreen'
  }
} as ComponentMeta<typeof Mandelbrot>

const Template: ComponentStory<typeof Mandelbrot> = args => {
  return <Mandelbrot {...args} />
}

export const Image = Template.bind({})
Image.args = {}
