#[cfg(test)]
mod tests {

    use dragonfly::vertex::{Figure, Mesh};

    #[test]
    fn test_triangle_vertices_and_indices() {
        let figure = Figure::Triangle;
        let vertices = figure.get_vertices();
        let indices = figure.get_indices();
        assert_eq!(vertices.len(), 3);
        assert_eq!(indices.len(), 3);
    }

    #[test]
    fn test_pentagon_vertices_and_indices() {
        let figure = Figure::Pentagon;
        let vertices = figure.get_vertices();
        let indices = figure.get_indices();
        assert_eq!(vertices.len(), 5);
        assert_eq!(indices.len(), 9);
    }

    #[test]
    fn test_rectangle_vertices_and_indices() {
        let figure = Figure::Rectangle;
        let vertices = figure.get_vertices();
        let indices = figure.get_indices();
        assert_eq!(vertices.len(), 4);
        assert_eq!(indices.len(), 6);
    }

    #[test]
    fn test_trapezoid_vertices_and_indices() {
        let figure = Figure::Trapezoid;
        let vertices = figure.get_vertices();
        let indices = figure.get_indices();
        assert_eq!(vertices.len(), 4);
        assert_eq!(indices.len(), 6);
    }

    #[test]
    fn test_parallelogram_vertices_and_indices() {
        let figure = Figure::Parallelogram;
        let vertices = figure.get_vertices();
        let indices = figure.get_indices();
        assert_eq!(vertices.len(), 4);
        assert_eq!(indices.len(), 6);
    }

    #[test]
    fn test_circle_vertices_and_indices() {
        let figure = Figure::Circle(64);
        let vertices = figure.get_vertices();
        let indices = figure.get_indices();
        assert_eq!(vertices.len(), 66);
        assert_eq!(indices.len(), 192);
    }
}
