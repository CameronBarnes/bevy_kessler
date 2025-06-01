use bevy::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Music>();
    app.register_type::<SoundEffect>();

    app.add_systems(
        Update,
        apply_global_volume.run_if(resource_changed::<GlobalVolume>),
    );

    app.add_observer(on_start_music);
    app.add_observer(on_next_song);
    app.add_observer(on_music_removed);
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "music" category (e.g. global background music, soundtrack).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Music;

/// A music audio instance.
pub fn music(handle: Handle<AudioSource>) -> impl Bundle {
    (
        AudioPlayer(handle),
        PlaybackSettings::DESPAWN,
        Music,
        StateScoped(Screen::Gameplay),
    )
}

/// Stores a list of music to play and an index for the current song
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Playlist {
    music: Vec<Handle<AudioSource>>,
    index: usize,
}

impl Playlist {
    /// Creates a new playlist of music
    pub fn new(handles: Vec<Handle<AudioSource>>) -> Self {
        info!("Creating new playlist");
        Self {
            music: handles,
            index: 0,
        }
    }
}

#[derive(Event)]
pub struct StartMusic;

fn on_start_music(
    _trigger: Trigger<StartMusic>,
    mut commands: Commands,
    mut playlist_query: Query<&mut Playlist>,
) {
    if let Ok(mut playlist) = playlist_query.single_mut() {
        playlist.index = usize::MAX;
        commands.trigger(NextSong);
        info!("Starting music!");
    }
}

#[derive(Event)]
pub struct NextSong;

fn on_next_song(
    _trigger: Trigger<NextSong>,
    mut commands: Commands,
    mut playlist_query: Query<&mut Playlist>,
) {
    if let Ok(mut playlist) = playlist_query.single_mut() {
        if playlist.index >= playlist.music.len() {
            playlist.index = 0;
        }
        info!("Next song!");
        commands.spawn(music(playlist.music[playlist.index].clone()));
    }
}

fn on_music_removed(
    _trigger: Trigger<OnRemove, Music>,
    mut commands: Commands,
    state: Res<State<Screen>>,
) {
    if matches!(state.get(), Screen::Gameplay) {
        commands.trigger(NextSong);
    }
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "sound effect" category (e.g. footsteps, the sound of a magic spell, a door opening).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct SoundEffect;

/// A sound effect audio instance.
pub fn sound_effect(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::DESPAWN, SoundEffect)
}

/// [`GlobalVolume`] doesn't apply to already-running audio entities, so this system will update them.
fn apply_global_volume(
    global_volume: Res<GlobalVolume>,
    mut audio_query: Query<(&PlaybackSettings, &mut AudioSink)>,
) {
    for (playback, mut sink) in &mut audio_query {
        sink.set_volume(global_volume.volume * playback.volume);
    }
}
