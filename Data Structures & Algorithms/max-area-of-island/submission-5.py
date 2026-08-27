class Solution:
    def maxAreaOfIsland(self, grid: List[List[int]]) -> int:
        R, C = len(grid), len(grid[0])
        
        # Flatten into a 1D array
        flat = [val for row in grid for val in row]
        max_area = 0
        
        for i in range(len(flat)):
            if flat[i]:
                flat[i] = 0
                area = 0
                stack = [i]
                
                while stack:
                    curr = stack.pop()
                    area += 1
                    
                    # Unrolled directional checks using 1D index math
                    # Down
                    down = curr + C
                    if down < len(flat) and flat[down]:
                        flat[down] = 0
                        stack.append(down)
                    
                    # Up
                    up = curr - C
                    if up >= 0 and flat[up]:
                        flat[up] = 0
                        stack.append(up)
                        
                    # Right (ensure not wrapping to next row)
                    if (curr % C) < C - 1:
                        right = curr + 1
                        if flat[right]:
                            flat[right] = 0
                            stack.append(right)
                            
                    # Left (ensure not wrapping to previous row)
                    if (curr % C) > 0:
                        left = curr - 1
                        if flat[left]:
                            flat[left] = 0
                            stack.append(left)
                
                if area > max_area:
                    max_area = area
                    
        return max_area