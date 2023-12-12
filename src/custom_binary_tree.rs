
#[derive(Debug)]
pub struct Node<T>{
    value: T,
    row:Vec<usize>,
    right: Option<Box<Node<T>>>,
    left:Option<Box<Node<T>>>,
}
impl<T: Clone> Node<T>{
    fn get_all(&self) -> Vec<T>{
        let mut current_values = Vec::<T>::new();
        if self.left.is_some(){
            current_values.append(&mut (*self.left.as_ref().unwrap()).get_all())
        }
        current_values.push(self.value.clone());
        if self.right.is_some(){
            current_values.append(&mut (*self.right.as_ref().unwrap()).get_all())
        }

        return  current_values;
    }
}
#[derive(Debug)]
pub struct ColumnBinaryTree<T:std::cmp::PartialEq+std::cmp::PartialOrd+Clone+std::fmt::Debug>{
    pub root:Option<Node<T>>,
    pub size:usize,
}
impl<T:std::cmp::PartialEq+std::cmp::PartialOrd+Clone+std::fmt::Debug> ColumnBinaryTree<T>{
    pub fn push(&mut self,value:T,row:usize) -> bool{
        if self.root.is_none(){
            //no root node generate and create one
            let root = Node::<T>{value,row:vec![row],right:None,left:None};
            self.root = Some(root);
        }else{
            
            let mut current_node = self.root.as_mut().unwrap();
            
            loop {

                if current_node.value == value{
                    //values match.
                    //add row to row vec
                    current_node.row.push(row);
                    self.size += 1;
                    //exit loop
                    return true;
                }else if current_node.value < value {
                    //go down the right node
                    //first check if None
                    if current_node.right.is_none(){
                        //just create a new boxed node
                        let new_node = Node::<T>{value,row:vec![row],left:None,right:None};
                        current_node.right = Some(Box::new(new_node));
                        self.size += 1;
                        return true;

                    }else{
                        //set current node to the right node and continue on with your life
                        current_node = &mut *(current_node.right.as_mut().unwrap());
                        continue;
                    }
                }else if current_node.value > value {
                    //go down the left node
                    //first check if none
                    if current_node.left.is_none(){
                        //just create a new boxed node
                        let new_node = Node::<T>{value,row:vec![row],left:None,right:None};
                        current_node.left = Some(Box::new(new_node));
                        self.size += 1;
                        return true;

                    }else{
                        //set current node to the left node and continue on with your life
                        current_node = &mut *(current_node.left.as_mut().unwrap());
                        continue;
                    }                   
                }else{
                    //something went wrong.
                    //what? not sure
                    return false;
                }
            }
        }

        true
    }    
    pub fn print(&self){
        //whilst im trying to not do things recursively i cant find a way not to for print   
        println!("{:?}",self.root.as_ref().unwrap().get_all()); 
    } 

}




