use std::mem;

struct MyLinkedList{
    head:Link,
    size:i32,
}
type Link=Option<Box<Node>>;

struct Node{
    value:i32,
    next:Link,
}

impl MyLinkedList {
    fn new()-> Self{
        MyLinkedList{head:None,size:0}
    }
    fn push(&mut self,val: i32){
        let head=Box::new(Node{
            value:val,
            next:self.head.take(),
        });
        self.head=Some(head);
        self.size+=1;
    }

    fn pop(&mut self)->Option<i32>{
       self.head.take().map(|node|{
            self.head=node.next;
            self.size-=1;
            node.value
        })
    }
    
    fn peek(&mut self)->Option<&i32>{
        self.head.as_ref().map(|node|{
            &node.value
        })
    }
    fn peek_mut(&mut self)->Option<&mut i32>{
        self.head.as_mut().map(|node|{
            &mut node.value
        })
    }

    fn into_iter(self)->IntoIter{
        return IntoIter(self)
    }

    pub fn iter(&mut self) -> Iter<'_> {
        Iter { next: self.head.as_deref() }
    }
}

// 返回不可变借用
pub struct Iter<'a> {
    next: Option<&'a Node>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.value
        })
    }
}

// 获取所有权
pub struct IntoIter(MyLinkedList);

impl Iterator for IntoIter {
    type Item=i32;
    fn next(&mut self)->Option<Self::Item>{
        self.0.pop()
    }
}

impl Drop for MyLinkedList{
    fn drop (&mut self){
        let mut cur_link= self.head.take();
        while let Some(mut node)=cur_link{
            cur_link=node.next.take();
        }
    }
}
#[cfg(test)]
mod test {
    use super::MyLinkedList;
    #[test]
    fn into_iter() {
        let mut list = MyLinkedList::new();
        list.push(1); list.push(2); list.push(3);
        assert_eq!(list.size,3);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    
    }

    #[test]
    fn iter() {
        let mut list = MyLinkedList::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
    
    #[test]
    fn test_into_iter(){
        let  arr=vec![1,2,3];
        for val in arr.iter(){
            println!("val:{}",val)
        }
        println!("arr{:?}",arr)
    }

    #[test]
    fn deref(){
        let a=Some(Box::new(1));
        let b=a.as_deref();
        println!("a:{}",b.unwrap())
    }
}