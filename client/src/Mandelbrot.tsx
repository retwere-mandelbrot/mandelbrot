import React, { useRef, useEffect } from 'react'
import init, { render_mandelbrot } from 'fractal-web'

async function render(x: number, y: number, zoom: number, context: CanvasRenderingContext2D) {
  try {
    await init()
    render_mandelbrot(zoom, BigInt(x), BigInt(y), context)
  } catch (e) {
    console.error(e)
  }
}

type MandelbrotProps = {
  x: number
  y: number
  zoom: number
}

function Mandelbrot({ x, y, zoom }: MandelbrotProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null)

  useEffect(() => {
    const canvas = canvasRef.current
    const context = canvas?.getContext('2d')!
    render(x, y, zoom, context)
  }, [x, y, zoom])

  return <canvas ref={canvasRef} width={512} height={512} />
}
export default Mandelbrot
