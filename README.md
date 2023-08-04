# ray-tracing

origin: https://raytracing.github.io/books/RayTracingInOneWeekend.html

## 4.1 

- 接收屏幕与实物是在原点的两侧的。z 轴正方向朝外。
- 为什么 j 是逆向枚举的：好吧 ppm 格式理所当然的是从左到右，从上到下描述的。作者只是想要 green 成分从 1 到 0 而已。噢不对，在 4.1 这里，i 横递增 0 到 1，j 纵递减 1 到 0，确实只有这样才符合屏幕上的成像。逆向枚举本质上是因为三维几何的 y 轴方向与 ppm 的纵向正方向相反。
