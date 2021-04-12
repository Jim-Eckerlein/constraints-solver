//
//  World.swift
//  ConstraintsSolver
//
//  Created by Jim on 08.04.21.
//

import Foundation

class World {
    private let integrator = SubStepIntegrator(subStepCount: 50)
    private let cubeMesh: Mesh
    private let cube: Rigid
    private let ground: Rigid
    
    init(renderer: Renderer) {
        cubeMesh = Mesh.makeCube(name: "Cube", color: .white)
        cubeMesh.map { $0 - simd_float3(0.5, 0.5, 0.5) }
        renderer.registerMesh(cubeMesh)
        
        cube = Rigid(collider: .box(BoxCollider()), mass: 1)
        cube.frame.quaternion = Quaternion(by: .pi / 8, around: .ey + 0.5 * .ex)
        cube.frame.position = Point(0, -2, 4)
        cube.externalForce = -9.81 * .ez
        cube.angularVelocity = Point(1, 2, 0.1)
        cube.velocity = 4 * .ey
        
        ground = Rigid(collider: .plane(Plane(direction: .ez, offset: 0)), mass: nil)
    }
    
    func integrate(dt: Double) {
        integrator.integrate([cube, ground], by: dt)
        cubeMesh.transform = cube.frame.matrix
    }
}
