
use std::collections::{VecDeque};

use sdl2::video::{self, Window, WindowContext};
use sdl2::render::{TextureCreator, Canvas};
use sdl2::pixels;

use log::debug;
use hecs::World;

use crate::resources::GMResources;
use crate::input::{GMInput, GMEventCode};
use crate::scene::{GMSceneT, GMSceneManagerMessage};
use crate::engine::GMEngineMessage;
use crate::configuration::GMConfiguration;
use crate::math::{GMPosition, GMVelocity, GMAcceleration, GMAngle, GMAngleVelocity, GMScale, GMFlipXY};
use crate::util::{GMActive, GMRepetition, GMVisible};
use crate::animation::GMAnimation;
use crate::texture::{GMTextureIndex, GMSharedTexture};
use crate::interpolation::{GMInterpolatePosition, GMInterpolateRotation};


pub struct GMContext {
    engine_messages: VecDeque<GMEngineMessage>,
    scene_messages: VecDeque<GMSceneManagerMessage>,
    canvas: Canvas<Window>,
    input: GMInput,
    resources: GMResources,
    window_width: f32,
    window_height: f32,
    world: World,
}

impl GMContext {
    pub(crate) fn new (texture_creator: TextureCreator<WindowContext>,
            event_pump: sdl2::EventPump, canvas: Canvas<Window>, configuration: &GMConfiguration) -> Self {
        let input = GMInput::new(event_pump);
        let resources = GMResources::new(texture_creator);

        Self {
            engine_messages: VecDeque::new(),
            scene_messages: VecDeque::new(),
            canvas,
            input,
            resources,
            window_width: configuration.screen_width as f32,
            window_height: configuration.screen_height as f32,
            world: World::new(),
        }
    }

    // Resources:
    pub fn resources(&self) -> &GMResources {
        &self.resources
    }

    pub fn resources_mut(&mut self) -> &mut GMResources {
        &mut self.resources
    }

    // ECS world
    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    // Engine messages:
    pub(crate) fn next_engine_message(&mut self) -> Option<GMEngineMessage> {
        self.engine_messages.pop_front()
    }

    pub fn quit(&mut self) {
        self.engine_messages.push_back(GMEngineMessage::Quit);
    }

    pub fn change_fps(&mut self, fps: u32) {
        self.engine_messages.push_back(GMEngineMessage::ChangeFPS(fps));
    }

    pub fn change_resolution(&mut self, width: u32, height: u32) {
        todo!("change_resolution: '{}', '{}'", width, height);
    }

    pub fn change_title<T: Into<String>>(&mut self, title: T) {
        todo!("change_title: '{}'", title.into());
    }


    // Scene messages:
    pub(crate) fn next_scene_message(&mut self) -> Option<GMSceneManagerMessage> {
        self.scene_messages.pop_front()
    }

    pub fn add_scene<T: 'static + GMSceneT, S: Into<String>>(&mut self, name: S, scene: T) {
        self.add_scene2(name.into(), Box::new(scene));
    }

    pub fn add_scene2<S: Into<String>>(&mut self, name: S, scene: Box<dyn GMSceneT>) {
        self.scene_messages.push_back(GMSceneManagerMessage::AddScene(name.into(), scene));
    }

    pub fn remove_scene<S: Into<String>>(&mut self, name: S) {
        self.scene_messages.push_back(GMSceneManagerMessage::RemoveScene(name.into()));
    }

    pub fn change_to_scene<S: Into<String>>(&mut self, name: S) {
        self.scene_messages.push_back(GMSceneManagerMessage::ChangeToScene(name.into()));
    }

    pub fn replace_scene<T: 'static + GMSceneT, S: Into<String>>(&mut self, name: S, scene: T) {
        self.replace_scene2(name.into(), Box::new(scene));
    }

    pub fn replace_scene2<S: Into<String>>(&mut self, name: S, scene: Box<dyn GMSceneT>) {
        self.scene_messages.push_back(GMSceneManagerMessage::ReplaceScene(name.into(), scene));
    }

    pub fn push_and_change_scene<S: Into<String>>(&mut self, name: S) {
        self.scene_messages.push_back(GMSceneManagerMessage::PushAndChangeScene(name.into()));
    }

    pub fn pop_and_change_scene(&mut self) {
        self.scene_messages.push_back(GMSceneManagerMessage::PopAndChangeScene);
    }

    // Update context, called by engine once per frame
    pub(crate) fn update(&mut self) {
        self.input.update();

    }

    // Events, called by user code
    pub fn event(&self, event_code: GMEventCode) -> bool {
        self.input.event(event_code)
    }

    // Called by engine every frame
    pub(crate) fn present(&mut self) {
        self.canvas.present();
    }

    pub fn clear_black(&mut self) {
        self.clear(pixels::Color::BLACK);
    }

    pub fn clear(&mut self, clear_color: pixels::Color) {
        self.canvas.set_draw_color(clear_color);
        self.canvas.clear();
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        debug!("GMContext::set_fullscreen(), fullscreen: '{}'", fullscreen);

        if fullscreen {
            self.canvas.window_mut().set_fullscreen(video::FullscreenType::True)
                .expect("GMContext::set_fullscreen(), could not set fullscreen on");
        } else {
            self.canvas.window_mut().set_fullscreen(video::FullscreenType::Off)
                .expect("GMContext::set_fullscreen(), could not set fullscreen off");
        }
    }

    pub fn window_width(&self) -> f32 {
        self.window_width
    }

    pub fn window_height(&self) -> f32 {
        self.window_height
    }

    // ECS methods
    pub fn draw_textures(&mut self) {
        let world = &self.world;
        let canvas = &mut self.canvas;

        for (_e, (texture,
            index,
            position,
            scale,
            angle,
            flip_xy,
            visible
            )) in
            world.query::<(&GMSharedTexture,
                &GMTextureIndex,
                &GMPosition,
                &GMScale,
                &GMAngle,
                &GMFlipXY,
                &GMVisible
                )>().iter() {
            if visible.0 {
                let v = position.0;
                let x = v.x;
                let y = v.y;
                let (sdl_texture, src_rect, dst_rect) = texture.0.get_draw_settings(x, y, index.0, scale.0);

                canvas.copy_ex(sdl_texture, src_rect, dst_rect, angle.0 as f64, None, flip_xy.0, flip_xy.1)
                .expect("GMContext::draw_texture_opt(), error when drawing texture!");
            }
        }
    }

    pub fn update_movables(&mut self) {
        self.process_animations();

        self.move_positions();
        self.accelerate_velocities();
        self.rotate_angles();

        self.interpolate_rotation();
        self.interpolate_position();
    }

    pub fn move_positions(&mut self) {
        for (_e, (position,
            velocity,
            active
            )) in
            self.world.query_mut::<(
                &mut GMPosition,
                &GMVelocity,
                &GMActive
            )>() {
            if active.0 {
                position.0.add2(&velocity.0);
            }
        }
    }

    pub fn accelerate_velocities(&mut self) {
        for (_e, (velocity,
            acceleration,
            active
            )) in
            self.world.query_mut::<(
                &mut GMVelocity,
                &GMAcceleration,
                &GMActive
            )>() {
            if active.0 {
                velocity.0.add2(&acceleration.0);
            }
        }
    }

    pub fn rotate_angles(&mut self) {
        for (_e, (angle,
            angle_velocity,
            active
            )) in
            self.world.query_mut::<(
                &mut GMAngle,
                &GMAngleVelocity,
                &GMActive
            )>() {
            if active.0 {
                angle.0 += &angle_velocity.0;
            }
        }
    }

    pub fn process_animations(&mut self) {
        for (_, (animation,
                 texture_index,
                 active)) in
            self.world.query_mut::<(&mut GMAnimation, &mut GMTextureIndex, &mut GMActive)>() {
            if active.0 && animation.timer.finished() {
                match animation.repetition {
                    GMRepetition::OnceForward => {
                        if animation.frame_at_end() {
                            active.0 = false;
                        } else {
                            animation.current_frame += 1;
                            animation.set_new_timer_duration();
                            texture_index.0 = animation.texture_index();
                        }
                    }
                    GMRepetition::OnceBackward => {
                        if animation.frame_at_start() {
                            active.0 = false;
                        } else {
                            animation.current_frame -= 1;
                            animation.set_new_timer_duration();
                            texture_index.0 = animation.texture_index();
                        }
                    }
                    GMRepetition::LoopForward => {
                        if animation.frame_at_end() {
                            // Restart animation
                            animation.current_frame = 0;
                        } else {
                            animation.current_frame += 1;
                        }
                        animation.set_new_timer_duration();
                        texture_index.0 = animation.texture_index();
                    }
                    GMRepetition::LoopBackward => {
                        if animation.frame_at_start() {
                            // Restart animation
                            animation.current_frame = animation.frames.len() - 1;
                        } else {
                            animation.current_frame -= 1;
                        }
                        animation.set_new_timer_duration();
                        texture_index.0 = animation.texture_index();
                    }
                    GMRepetition::PingPongForward => {
                        if animation.frame_at_end() {
                            animation.repetition =  GMRepetition::PingPongBackward;
                        } else {
                            animation.current_frame += 1;
                        }
                        animation.set_new_timer_duration();
                        texture_index.0 = animation.texture_index();
                    }
                    GMRepetition::PingPongBackward => {
                        if animation.frame_at_start() {
                            animation.repetition =  GMRepetition::PingPongForward;
                        } else {
                            animation.current_frame -= 1;
                        }
                        animation.set_new_timer_duration();
                        texture_index.0 = animation.texture_index();
                    }
                }
            }
        }
    }

    pub fn interpolate_rotation(&mut self) {
        for (_e, (angle,
            interpolate,
            active
            )) in
            self.world.query_mut::<(
                &mut GMAngle,
                &mut GMInterpolateRotation,
                &GMActive
            )>() {
            if active.0 {
                let interpolate = &mut interpolate.0;
                interpolate.update();
                angle.0 = interpolate.get_value();
            }
        }
    }

    pub fn interpolate_position(&mut self) {
        for (_e, (position,
            interpolate,
            active
            )) in
            self.world.query_mut::<(
                &mut GMPosition,
                &mut GMInterpolatePosition,
                &GMActive
            )>() {
            if active.0 {
                let interpolate = &mut interpolate.0;
                interpolate.update();
                position.0 = interpolate.get_vector();
            }
        }
    }


}
