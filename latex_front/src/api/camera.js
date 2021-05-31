import { CHANGE_CAMERA, DRAGGING_CAMERA } from "state/camera/action";
import { ZOOM_CAMERA } from "state/camera/action";
import {RESET_CAMERA} from "state/camera/action";

export function camera_vec(x,y) {
    return {
        type: CHANGE_CAMERA,
        x: x,
        y: y,
    };
}

export function zoom_camera(zoom){
    return {
        type: ZOOM_CAMERA,
        zoom: zoom 
    }
}

export function reset_camera(reset){
    return {
        type: RESET_CAMERA,
        r: reset
    }
}

export function is_dragging(dragging){
    return {
        type: DRAGGING_CAMERA,
        is_dragging: dragging
    }
}