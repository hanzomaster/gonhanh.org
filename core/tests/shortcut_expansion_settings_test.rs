use gonhanh_core::data::keys;
use gonhanh_core::engine::{shortcut::Shortcut, Engine};

fn add_word_shortcut(engine: &mut Engine) {
    engine.shortcuts_mut().add(Shortcut::new("vn", "Việt Nam"));
}

fn type_vn_and_space(engine: &mut Engine) -> u8 {
    engine.on_key(keys::V, false, false);
    engine.on_key(keys::N, false, false);
    engine.on_key(keys::SPACE, false, false).action
}

#[test]
fn master_switch_blocks_shortcuts_in_both_modes() {
    let mut engine = Engine::new();
    add_word_shortcut(&mut engine);
    engine.set_shortcut_expansion(false, true, true);

    assert_eq!(type_vn_and_space(&mut engine), 0);

    engine.set_enabled(false);
    assert_eq!(type_vn_and_space(&mut engine), 0);
}

#[test]
fn vietnamese_only_setting_follows_engine_mode() {
    let mut engine = Engine::new();
    add_word_shortcut(&mut engine);
    engine.set_shortcut_expansion(true, false, true);

    assert_eq!(type_vn_and_space(&mut engine), 1);

    engine.set_enabled(false);
    assert_eq!(type_vn_and_space(&mut engine), 0);
}

#[test]
fn english_only_setting_follows_engine_mode() {
    let mut engine = Engine::new();
    add_word_shortcut(&mut engine);
    engine.set_shortcut_expansion(true, true, false);

    assert_eq!(type_vn_and_space(&mut engine), 0);

    engine.set_enabled(false);
    assert_eq!(type_vn_and_space(&mut engine), 1);
}

#[test]
fn disabling_shortcuts_keeps_vietnamese_transforms_active() {
    let mut engine = Engine::new();
    engine.set_shortcut_expansion(false, false, false);

    engine.on_key(keys::A, false, false);
    let result = engine.on_key(keys::S, false, false);

    assert_eq!(result.action, 1);
    assert_eq!(result.chars[0], 'á' as u32);
}

#[test]
fn changing_scope_clears_partial_shortcut() {
    let mut engine = Engine::new();
    engine.set_enabled(false);
    engine
        .shortcuts_mut()
        .add(Shortcut::new("btw", "by the way"));

    engine.on_key(keys::B, false, false);
    engine.on_key(keys::T, false, false);
    engine.set_shortcut_expansion(true, false, true);
    engine.set_shortcut_expansion(true, true, true);
    engine.on_key(keys::W, false, false);

    assert_eq!(engine.on_key(keys::SPACE, false, false).action, 0);
}

#[test]
fn option_key_clears_composition_when_shortcuts_are_disabled() {
    let mut engine = Engine::new();
    engine.on_key(keys::A, false, false);
    engine.set_shortcut_expansion(false, true, true);

    engine.on_key_with_char(keys::V, false, true, false, Some('√'));

    assert_eq!(engine.debug_buffer_len(), 0);
}

#[test]
fn special_character_shortcuts_respect_master_switch() {
    let mut engine = Engine::new();
    engine.shortcuts_mut().add(Shortcut::immediate("√√", "✅"));
    engine.set_shortcut_expansion(false, true, true);

    engine.on_key_with_char(keys::V, false, true, false, Some('√'));
    assert_eq!(
        engine
            .on_key_with_char(keys::V, false, true, false, Some('√'))
            .action,
        0
    );

    engine.set_shortcut_expansion(true, true, true);
    engine.on_key_with_char(keys::V, false, true, false, Some('√'));
    assert_eq!(
        engine
            .on_key_with_char(keys::V, false, true, false, Some('√'))
            .action,
        1
    );
}
