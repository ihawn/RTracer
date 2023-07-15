use crate::datatypes::vector2d::Vector2D;
use crate::datatypes::color::Color;


pub fn remove_fireflies(color_matrix: &Vector2D<Color>) -> Vector2D<Color> {
    let mut new_colors: Vector2D<Color> = Vector2D::new(color_matrix.width, color_matrix.height, Color::black());
    let idx_m: [i8; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
    let idx_n: [i8; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];

    for i in 1..color_matrix.height - 1 {
        for j in 1..color_matrix.width - 1 {
            let current_color: Color = *color_matrix.get(i, j);

            let mut similar_found = false;
            let mut neighbor_count = 0;
            let mut color_sum = Color::black();

            for n in 0..8 {
                let neighbor_color: Color = *color_matrix.get(i + idx_m[n] as usize, j + idx_n[n] as usize);
                if (current_color.to_vector3() - neighbor_color.to_vector3()).magnitude() < 0.2 {
                    similar_found = true;
                    break;
                }

                color_sum += neighbor_color;
                neighbor_count += 1;
            }

            if !similar_found && neighbor_count > 0 {
                let average_color = color_sum * (1.0 / neighbor_count as f32);
                new_colors.set(i, j, average_color);
            } else {
                new_colors.set(i, j, current_color);
            }
        }
    }

    new_colors
}
