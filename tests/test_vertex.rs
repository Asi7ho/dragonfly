#[cfg(test)]
mod tests {

    use dragonfly::vertex::Figure;

    #[test]
    fn test_figure_get_vertices_and_indices() {
        let triangle = Figure::Triangle;
        let (vertices, indices) = triangle.get_vertices_and_indices();
        assert_eq!(vertices.len(), 3);
        assert_eq!(indices.len(), 3);

        let pentagon = Figure::Pentagon;
        let (vertices, indices) = pentagon.get_vertices_and_indices();
        assert_eq!(vertices.len(), 5);
        assert_eq!(indices.len(), 9);

        let rectangle = Figure::Rectange;
        let (vertices, indices) = rectangle.get_vertices_and_indices();
        assert_eq!(vertices.len(), 4);
        assert_eq!(indices.len(), 6);

        let trapezoid = Figure::Trapezoid;
        let (vertices, indices) = trapezoid.get_vertices_and_indices();
        assert_eq!(vertices.len(), 4);
        assert_eq!(indices.len(), 6);

        let parallelogram = Figure::Parallelogram;
        let (vertices, indices) = parallelogram.get_vertices_and_indices();
        assert_eq!(vertices.len(), 4);
        assert_eq!(indices.len(), 6);

        let circle = Figure::Circle; // 64 segments
        let (vertices, indices) = circle.get_vertices_and_indices();
        assert_eq!(vertices.len(), 66);
        assert_eq!(indices.len(), 192);
    }
}
