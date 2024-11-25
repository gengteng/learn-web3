use anchor_lang::prelude::*;

declare_id!("cyB9z9K3XP9eWsCFBiigjnZKaYXbQL4ULUKuAMbyTGm");

#[program]
pub mod hello_anchor {
    use super::*;

    pub fn create_todo_list(ctx: Context<CreateTodoList>) -> Result<()> {
        ctx.accounts.todo_list.owner = *ctx.accounts.signer.key;
        msg!("Create a new todo list for {}", ctx.accounts.signer.key());
        Ok(())
    }

    pub fn add_todo_item(ctx: Context<ModifyTodoList>, item: String) -> Result<()> {
        let todo_list = &mut ctx.accounts.todo_list;
        if ctx.accounts.signer.key() != todo_list.owner {
            return Err(ErrorCode::Unauthorized.into());
        }
        todo_list.items.push(item);
        msg!("Added a new todo item");
        Ok(())
    }

    pub fn remove_todo_item(ctx: Context<ModifyTodoList>, item: String) -> Result<()> {
        let todo_list = &mut ctx.accounts.todo_list;
        if ctx.accounts.signer.key() != todo_list.owner {
            return Err(ErrorCode::Unauthorized.into());
        }
        todo_list.items.retain(|i| i != &item);
        msg!("Removed a todo item");
        Ok(())
    }

    pub fn query_todo_items(ctx: Context<QueryTodoList>) -> Result<Vec<String>> {
        let todo_list = &ctx.accounts.todo_list;
        if ctx.accounts.signer.key() != todo_list.owner {
            return Err(ErrorCode::Unauthorized.into());
        }
        Ok(todo_list.items.clone())
    }
}

#[derive(Accounts)]
pub struct CreateTodoList<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(init, payer = signer, space = 8 + TodoList::INIT_SPACE)]
    pub todo_list: Account<'info, TodoList>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)] // automatically calculate the space required for the struct
pub struct TodoList {
    pub owner: Pubkey, // 32 bytes
    #[max_len(50, 100)] // set a max length for the string
    pub items: Vec<String>, // 4 bytes + 50 bytes
}

#[derive(Accounts)]
pub struct ModifyTodoList<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub todo_list: Account<'info, TodoList>,
}

#[derive(Accounts)]
pub struct QueryTodoList<'info> {
    pub signer: Signer<'info>,
    pub todo_list: Account<'info, TodoList>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
}
