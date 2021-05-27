import { CHANGE_CAMERA } from "state/camera/action";
import { ZOOM_CAMERA } from "state/camera/action";
export function camera_vec(x,y,z) {
    return {
        type: CHANGE_CAMERA,
        x: x,
        y: y,
        z: z
    };
}

export function zoom_camera(zoom){
    return {
        type: ZOOM_CAMERA,
        zoom: zoom 
    }
}