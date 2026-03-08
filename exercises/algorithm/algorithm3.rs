/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

/// 通用冒泡排序实现
/// 约束：T 必须实现 Ord 特征（支持大小比较）
fn sort<T>(array: &mut [T]) where T: Ord {
    let n = array.len();
    // 边界条件：空数组或单个元素，无需排序
    if n <= 1 {
        return;
    }

    // 外层循环：控制排序轮数（每轮将最大的元素"冒泡"到末尾）
    for i in 0..n {
        let mut swapped = false; // 标记本轮是否发生交换

        // 内层循环：遍历未排序部分，比较相邻元素
        // 每轮结束后，末尾 i 个元素已排序，无需再比较
        for j in 0..n - 1 - i {
            // 逆序则交换（升序排序）
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
                swapped = true;
            }
        }

        // 优化：如果本轮无交换，说明数组已完全有序，提前退出
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}