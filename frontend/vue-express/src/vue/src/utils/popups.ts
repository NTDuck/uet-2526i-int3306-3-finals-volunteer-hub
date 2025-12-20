import { createVNode, render } from 'vue'
import ErrorPopup from '../components/ErrorPopup.vue'
import ConfirmationPopup from '../components/ConfirmationPopup.vue'

export function showErrorPopup(errorTitle: string, errorMessage: string, errorWidth?: number) {
  if (typeof document === 'undefined') return

  const container = document.createElement('div')

  const destroy = () => {
    render(null, container)
    container.remove()
  }

  const vnode = createVNode(ErrorPopup, {
    title: errorTitle,
    message: errorMessage,
    width: errorWidth,
    onClose: destroy
  })

  render(vnode, container)
  document.body.appendChild(container)
}

export function showConfirmationPopup(title: string, message: string): Promise<boolean> {
  return new Promise((resolve) => {
    if (typeof document === 'undefined') {
      resolve(false)
      return
    }

    const container = document.createElement('div')

    const destroy = () => {
      render(null, container)
      container.remove()
    }

    const handleConfirm = () => {
      resolve(true)
      destroy()
    }

    const handleCancel = () => {
      resolve(false)
      destroy()
    }

    const vnode = createVNode(ConfirmationPopup, {
      title,
      message,
      onConfirm: handleConfirm,
      onCancel: handleCancel
    })

    render(vnode, container)
    document.body.appendChild(container)
  })
}
