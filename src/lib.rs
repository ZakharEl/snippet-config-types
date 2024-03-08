#![doc = include_str!("../README.md")]

///Structure to reduce typing through autocompleting a larger string of text from a smaller one.
#[stabby::stabby]
pub struct Snippet {
	///The smaller string used to trigger the autocompletion.
	pub trigger: stabby::string::String,
	///Text summarizing the larger autocompleted text (AKA the body below).
	///Typically used in a drop down menue to select a snippet without fully typing out the trigger above.
	/// Essentially all snippets with a trigger starting with the string typed out so far would be in this drop down menue.
	pub description: stabby::string::String,
	///The text autocompleted into existence at the end.
	pub body: stabby::string::String,
}

///Structure to limit a list of snippets to only be applicable when this structure is active. Therefore the the list of snippets are scoped and thus the name.
///Typically used to scope snippets to different file types, programming and markup languages, etc.
///This allows the same trigger (see [Snippet]) to autocomplete to different text depending upon the file types, etc it's written in.
///After all things like if statements have different structures depending on the programming language they are written in.
#[stabby::stabby]
pub struct Scope {
	///Typically identifies the programming or markup language that the snippets below are applicable within.
	pub name: stabby::string::String,
	///The snippets that can be trigger whereever this scope is applicable
	pub snippets: stabby::vec::Vec<Snippet>,
}

#[cfg(test)]
mod tests {
    use super::*;
}
