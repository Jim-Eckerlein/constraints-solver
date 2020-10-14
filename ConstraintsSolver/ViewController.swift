import Cocoa
import MetalKit

class ViewController: NSViewController, FrameDelegate {
    
    var renderer: Renderer!
    var mtkView: MTKView!
    var cube: Geometry!
    var triangle: Geometry!
    
    override func loadView() {
        mtkView = MTKView(frame: AppDelegate.windowRect)
        mtkView.device = MTLCreateSystemDefaultDevice()!
        
        renderer = Renderer(metalKitView: mtkView)
        mtkView.delegate = renderer
        
        renderer.frameDelegate = self
        renderer.cameraDistance = 6
        
        cube = renderer.makeCube(name: "Cube", color: .white)
        cube.transform = Transform(translation: simd_float3(), eulerAngles: simd_float3(0, 3.1415 * 0.25, 0))
        
        triangle = renderer.makeTriangle(name: "Triangle", colors: (.red, .green, .blue))
        triangle.transform.translation = -e1 + -e2
        
        let triangle2 = renderer.makeTriangle(name: "Triangle 2", colors: (.red, .yellow, .magenta))
        triangle2.transform.translation = e1 + -2 * e2
        
        self.view = mtkView
    }
    
    override func keyDown(with event: NSEvent) {
        if event.characters == "q" && event.modifierFlags.intersection(.deviceIndependentFlagsMask) == .command {
            NSApplication.shared.terminate(self)
        }
    }
    
    override func mouseDragged(with event: NSEvent) {
        renderer.cameraRotationAroundZ += Float(event.deltaX) * -0.01
        renderer.cameraRotationElevation += Float(event.deltaY) * -0.01
    }
    
    func onFrame(dt: Float, t: Float) {
        cube.transform.translation.z = sinf(0.5 * t)
        cube.transform.rotate(eulerAngles: simd_float3(1.5 * dt, 0, 0))
    }
}
