class Solution:
    def findOrder(self, numCourses: int, prerequisites: List[List[int]]) -> List[int]:
        graph = {}
        graph = {i: [] for i in range(numCourses)}
        for course, prerequisite in prerequisites:
            graph[course].append(prerequisite)
        
        path = set()
        visited = set()
        out = []
        def dfs(course):
            if course in path:
                return False
            if course in visited:
                return True
            
            path.add(course)
            for next_course in graph[course]:
                if not dfs(next_course):
                    return False
            path.remove(course)
            visited.add(course)
            out.append(course)
            return True
        print(out)
        return out if all(dfs(i) for i in range(numCourses)) else []