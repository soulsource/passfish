use std::sync::mpsc::{Receiver, RecvError};
use super::super::thread_messages::UiToHelper;

pub(super) fn receive_and_get_newest_or_important_command(receiver : &Receiver<UiToHelper>) -> Result<NewestMostImportantCommand, RecvError> {
    let first_message = receive_and_log_error(receiver)?;
    Ok(get_newest_or_important_command(first_message, receiver.try_iter()))
}

pub(super) struct NewestMostImportantCommand(pub UiToHelper);

trait GetNewerOrMoreImportantCommand {
    type Output;
    fn get_newer_or_more_important_command(old: Self::Output, new: Self) -> Self::Output;
    fn convert_first_command(self) -> Self::Output;
}

impl GetNewerOrMoreImportantCommand for UiToHelper {
    type Output = NewestMostImportantCommand;

    fn convert_first_command(self) -> Self::Output{
        NewestMostImportantCommand(self)
    }
    fn get_newer_or_more_important_command(old: Self::Output, new: Self) -> Self::Output {
        match old.0 {
            UiToHelper::Shutdown => old,
            UiToHelper::GeneratePassword(_) => NewestMostImportantCommand(new),
        }
    }
}


fn get_newest_or_important_command<I, It >(first_message: I, other_messages: It) -> I::Output
    where I: GetNewerOrMoreImportantCommand,
    It: Iterator<Item=I> 
{
    other_messages.fold(first_message.convert_first_command(), I::get_newer_or_more_important_command)
}

fn receive_and_log_error(receiver : &Receiver<UiToHelper>) -> Result<UiToHelper, RecvError> {
    match receiver.recv() {
        Ok(x) => Ok(x),
        e => {
                eprintln!("Connection to UI Thread closed unexpectedly.");
                e
             }
    }
}

#[cfg(test)]
mod message_parsing_tests {
    use super::*;

    #[derive(PartialEq, Debug)]
    enum TestPrioritizedEnum {
        Lowest(usize),
        Highest(usize),
        Medium(usize)
    }
    impl TestPrioritizedEnum{
        fn get_prio(&self) -> usize{
            match self {
                TestPrioritizedEnum::Lowest(_) => 0,
                TestPrioritizedEnum::Highest(_) => 2,
                TestPrioritizedEnum::Medium(_) => 1,
            }
        }
    }

    struct TestPrioritizedEnumPrioResult(TestPrioritizedEnum);
    impl GetNewerOrMoreImportantCommand for TestPrioritizedEnum {
        type Output = TestPrioritizedEnumPrioResult;

        fn get_newer_or_more_important_command(old: Self::Output, new: Self) -> Self::Output {
            if old.0.get_prio() > new.get_prio() { old } else { TestPrioritizedEnumPrioResult(new) }
        }

        fn convert_first_command(self) -> Self::Output {
            TestPrioritizedEnumPrioResult(self)
        }
    }

    #[test]
    fn test_get_newest_or_important_command_test(){
        let input1 = vec![TestPrioritizedEnum::Lowest(0), TestPrioritizedEnum::Lowest(1), TestPrioritizedEnum::Lowest(2)];
        let expected1 = TestPrioritizedEnum::Lowest(2);
        let input2 = vec![TestPrioritizedEnum::Medium(0), TestPrioritizedEnum::Lowest(1), TestPrioritizedEnum::Lowest(2)];
        let expected2 = TestPrioritizedEnum::Medium(0);
        let input3 = vec![TestPrioritizedEnum::Lowest(0), TestPrioritizedEnum::Medium(1), TestPrioritizedEnum::Lowest(2)];
        let expected3 = TestPrioritizedEnum::Medium(1);
        let input4 = vec![TestPrioritizedEnum::Medium(0), TestPrioritizedEnum::Lowest(1), TestPrioritizedEnum::Medium(2)];
        let expected4 = TestPrioritizedEnum::Medium(2);
        let input5 = vec![TestPrioritizedEnum::Lowest(0), TestPrioritizedEnum::Lowest(1), TestPrioritizedEnum::Highest(2), TestPrioritizedEnum::Medium(3)];
        let expected5 = TestPrioritizedEnum::Highest(2);
        let data = vec![(input1, expected1), (input2, expected2), (input3, expected3), (input4, expected4), (input5, expected5)];
        for (input, expected) in data {
            let mut it = input.into_iter();
            let first = it.next().unwrap();
            let result = get_newest_or_important_command(first,it);
            assert_eq!(result.0, expected);
        }
    }
}