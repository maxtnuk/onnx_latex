import * as THREE from 'three'

function Arrow(props) {
    const from=props.from;
    const length=props.length;
    const conn_rate=props.rate;

    const radius=5;
    const bar_geometry = new THREE.CylinderGeometry( radius, radius,32, length*conn_rate );
    const conn_geometry = new new THREE.ConeGeometry( radius+5, length*(1-conn_rate), 32 );
    
    return (
        <mesh
            {...props}
        >
            <mesh
                geometry={bar_geometry}>
                <meshBasicMaterial
                color={0xffff00}>
                </meshBasicMaterial>
            </mesh>
            <mesh
                geometry={conn_geometry}
            >
                <meshBasicMaterial
                color={0xffff00}>
                </meshBasicMaterial>
            </mesh>            
        </mesh>    
    );
}