import { useRef, useEffect } from 'react'
import init, { render_mandelbrot } from 'fractal-web'

async function f(context: CanvasRenderingContext2D) {
  try {
    await init()
    render_mandelbrot(0, BigInt(0), BigInt(0), context)
  } catch (e) {
    console.error(e)
  }
}

type MandelbrotProps = {
  res: number
  x_min: number
  x_max: number
  y_min: number
  y_max: number
}

function Mandelbrot({
  res = 512,
  x_min = -4.0,
  x_max = 4.0,
  y_min = -4.0,
  y_max = 4.0
}: MandelbrotProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null)

  useEffect(() => {
    const canvas = canvasRef.current
    const context = canvas?.getContext('2d')!
    // //Our first draw
    // context.fillStyle = '#ff00dd'
    // context.fillRect(0, 0, context.canvas.width, context.canvas.height)
    f(context)
  }, [])

  return <canvas ref={canvasRef} width={res} height={res} />
}
export default Mandelbrot
