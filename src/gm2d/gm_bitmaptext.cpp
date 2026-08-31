/*
    GreenMoon2D
    SPDX-License-Identifier: MIT
    Written by Willi Kappler, MIT License
    https://github.com/willi-kappler/green_moon_2d

    This file defines the bitmap text
*/

// Local includes:
#include "gm_bitmaptext.hpp"

namespace gm2d {
GMBitmapText::GMBitmapText(GMStringId obj_id, std::string text, std::shared_ptr<GMBitmapFont> font):
    GMObject(obj_id),
    bm_text(text),
    bm_font(font),
    bm_horizontal(true),
    bm_sine_effect(false),
    bm_shake_effect(false),
    bm_rotation_effect(false)
{}

void GMBitmapText::gm_handle_message(const GMObjectMessage &message, GMContext &context) {
    switch (message.msg_type) {
        case GMObjectMessageType::SetText:
            bm_text = std::any_cast<std::string>(message.msg_data);
        break;
        case GMObjectMessageType::GetText:
        {
            GMObjectMessage reply_message = GMObjectMessage(obj_name_id, message.msg_sender, GMObjectMessageType::GetTextResult)
                .with_msg_data(bm_text);
            context.gm_send_object_message(reply_message);
        }
        break;
        case GMObjectMessageType::SetFont:
            bm_font = std::any_cast<std::shared_ptr<GMBitmapFont>>(message.msg_data);
        break;
        case GMObjectMessageType::SetHorizontal:
            bm_horizontal = std::any_cast<bool>(message.msg_data);
        break;
        case GMObjectMessageType::SetSineEffect:
            bm_sine_effect = std::any_cast<bool>(message.msg_data);
        break;
        case GMObjectMessageType::SetShakeEffect:
            bm_shake_effect = std::any_cast<bool>(message.msg_data);
        break;
        case GMObjectMessageType::SetRotationEffect:
            bm_rotation_effect = std::any_cast<bool>(message.msg_data);
        break;
        default:
            GMObject::gm_handle_message(message, context);
        break;
    }
}

void GMBitmapText::gm_update([[maybe_unused]] GMContext &context) {
    // TODO: update text effects
    if (obj_active) {
        if (bm_sine_effect) {

        }

        if (bm_shake_effect) {

        }

        if (bm_rotation_effect) {

        }
    }
}

void GMBitmapText::gm_draw(GMContext &context) {
    float current_x = obj_position.x + obj_pos_delta.x;
    float current_y = obj_position.y + obj_pos_delta.y;
    float char_width = static_cast<float>(bm_font->gm_char_width());
    float char_height = static_cast<float>(bm_font->gm_char_height());

    if (obj_visible) {
        if (bm_horizontal) {
            current_x += char_width;
        } else {
            current_y += char_height;
        }

        for (char c: bm_text) {
            bm_font->gm_draw(context, current_x, current_y, c);
        }
    }

    obj_pos_delta.x = 0.0f;
    obj_pos_delta.y = 0.0f;
}

void GMBitmapText::gm_set_horizontal(bool horizontal) {
    bm_horizontal = horizontal;
}
}
