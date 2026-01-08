use leptos::prelude::*;

// * Reuse @dialog.rs
pub use crate::components::ui::dialog::{
    Dialog as AlertDialog, DialogBody as AlertDialogBody, DialogClose as AlertDialogClose, DialogContent,
    DialogDescription as AlertDialogDescription, DialogFooter as AlertDialogFooter, DialogHeader as AlertDialogHeader,
    DialogTitle as AlertDialogTitle, DialogTrigger as AlertDialogTrigger,
};

#[component]
pub fn AlertDialogContent(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <DialogContent class=class close_on_backdrop_click=false>
            {children()}
        </DialogContent>
    }
}