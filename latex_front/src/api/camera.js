import { CHANGE_CAMERA } from "state/camera/action";
export function camera_vec(x,y,z) {
    return {
        type: CHANGE_CAMERA,
        x: x,
        y: y,
        z: z
    };
}
