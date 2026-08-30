class Solution:
    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        result: List[List[int]] = []
        subset: List[int] = []
        candidates.sort()
        print(candidates)
        def explore(index, current_sum):
            if current_sum == target:
                print("added")
                result.append(subset.copy())
                return True
            if index == len(candidates) or current_sum + candidates[index] > target:
                return False

            current_num = candidates[index]
            # explore
            subset.append(current_num)
            
            explore(index+1, current_sum + current_num)


            #backtrack
            popped_num = subset.pop()
            while index+1 < len(candidates) and candidates[index+1] == popped_num:
                index += 1
            explore(index+1, current_sum)

        explore(0, 0)
        return result
