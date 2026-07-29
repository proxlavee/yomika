import { fireEvent, render, screen, waitFor } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { afterEach, describe, expect, it, vi } from 'vitest'

import { ColorPicker } from '@/components/ui/color-picker'

const { restoreAppWindowInteraction } = vi.hoisted(() => ({
  restoreAppWindowInteraction: vi.fn(async () => {}),
}))

vi.mock('@/lib/backend', () => ({ restoreAppWindowInteraction }))

// Stub react-colorful: its drag math depends on real layout/pointer capture,
// which jsdom cannot provide. The tests drive `onChange` directly to
// simulate a drag, then release the pointer outside the picker bounds —
// the exact scenario from the dropped-color bug.
vi.mock('react-colorful', () => ({
  HexColorPicker: ({ onChange }: { onChange: (color: string) => void }) => (
    <div data-testid='hex-picker' onClick={() => onChange('#000000')} />
  ),
  HexColorInput: ({
    color,
    onChange,
    prefixed,
    ...props
  }: {
    color: string
    onChange: (color: string) => void
    prefixed?: boolean
    [key: string]: unknown
  }) => (
    <input
      {...props}
      value={prefixed ? color : color.replace(/^#/, '')}
      onChange={(e) => onChange(e.target.value)}
    />
  ),
}))

describe('ColorPicker', () => {
  afterEach(() => {
    restoreAppWindowInteraction.mockClear()
  })

  it('commits the color when the pointer is released outside the picker', async () => {
    const onChange = vi.fn()
    render(
      <ColorPicker
        value='#FFFFFF'
        onChange={onChange}
        triggerTestId='trigger'
        pickerTestId='picker'
      />,
    )

    await userEvent.click(screen.getByTestId('trigger'))

    // Simulate a drag toward black: the picker reports the new color…
    fireEvent.click(screen.getByTestId('hex-picker'))
    // …but the pointer is released outside the picker element.
    fireEvent.pointerUp(document.body)

    expect(onChange).toHaveBeenCalledWith('#000000')
  })

  it('does not commit on unrelated pointer releases when nothing was dragged', async () => {
    const onChange = vi.fn()
    render(
      <ColorPicker
        value='#FFFFFF'
        onChange={onChange}
        triggerTestId='trigger'
        pickerTestId='picker'
      />,
    )

    await userEvent.click(screen.getByTestId('trigger'))
    fireEvent.pointerUp(document.body)

    expect(onChange).not.toHaveBeenCalled()
  })

  it('commits hex input edits immediately', async () => {
    const onChange = vi.fn()
    render(
      <ColorPicker
        value='#FFFFFF'
        onChange={onChange}
        triggerTestId='trigger'
        inputTestId='hex-input'
      />,
    )

    await userEvent.click(screen.getByTestId('trigger'))
    fireEvent.change(screen.getByTestId('hex-input'), { target: { value: '#123ABC' } })

    expect(onChange).toHaveBeenCalledWith('#123ABC')
  })

  describe('EyeDropper', () => {
    afterEach(() => {
      Reflect.deleteProperty(window, 'EyeDropper')
    })

    it('applies the picked color and settles cleanly', async () => {
      Object.assign(window, {
        EyeDropper: class {
          async open() {
            return { sRGBHex: '#abcdef' }
          }
        },
      })
      const onChange = vi.fn()
      render(
        <ColorPicker
          value='#FFFFFF'
          onChange={onChange}
          triggerTestId='trigger'
          pickButtonTestId='pick'
        />,
      )

      await userEvent.click(screen.getByTestId('trigger'))
      await userEvent.click(screen.getByTestId('pick'))

      await waitFor(() => expect(onChange).toHaveBeenCalledWith('#ABCDEF'))
      expect(restoreAppWindowInteraction).toHaveBeenCalledOnce()
    })

    it('aborting the EyeDropper neither throws nor changes the color', async () => {
      Object.assign(window, {
        EyeDropper: class {
          async open(): Promise<{ sRGBHex: string }> {
            throw new DOMException('cancelled', 'AbortError')
          }
        },
      })
      const onChange = vi.fn()
      const consoleError = vi.spyOn(console, 'error').mockImplementation(() => {})
      render(
        <ColorPicker
          value='#FFFFFF'
          onChange={onChange}
          triggerTestId='trigger'
          pickButtonTestId='pick'
        />,
      )

      await userEvent.click(screen.getByTestId('trigger'))
      await userEvent.click(screen.getByTestId('pick'))

      // Let the rejection settle, then assert nothing happened.
      await new Promise((resolve) => setTimeout(resolve, 50))
      expect(onChange).not.toHaveBeenCalled()
      expect(consoleError).not.toHaveBeenCalled()
      expect(restoreAppWindowInteraction).toHaveBeenCalledOnce()
    })
  })
})
