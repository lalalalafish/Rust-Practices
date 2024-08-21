fn deliver_order(){}
mod back_of_house{
    #[allow(unused)]
    fn fix_incorrect_order(){
        #[allow(unused)]
        cook_order();
        super::deliver_order();
    }   
    #[allow(unused)]
    fn cook_order(){}
}