use super::*;
use crate::mock::{new_test_ext, *};
use codec::{Decode, Encode};
use frame_support::{assert_err, assert_ok, traits::OnFinalize};
use sp_core::bounded_vec;

pub fn last_event() -> Event<Test> {
	System::events()
		.into_iter()
		.map(|r| r.event)
		.filter_map(|e| if let RuntimeEvent::Chainlink(inner) = e { Some(inner) } else { None })
		.last()
		.unwrap()
}

#[test]
fn operators_can_be_registered() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert!(!Chainlink::operator(1));
		assert_ok!(Chainlink::register_operator(RuntimeOrigin::signed(1)));
		assert_eq!(last_event(), Event::OperatorRegistered(1));
		assert!(Chainlink::operator(1));
		assert_ok!(Chainlink::unregister_operator(RuntimeOrigin::signed(1)));
		assert!(!Chainlink::operator(1));
		assert_eq!(last_event(), Event::OperatorUnregistered(1));
	});

	new_test_ext().execute_with(|| {
		assert_err!(
			Chainlink::unregister_operator(RuntimeOrigin::signed(1)),
			<Error<Test>>::UnknownOperator
		);
		assert!(!Chainlink::operator(1));
	});
}

#[test]
fn initiate_requests() {
	new_test_ext().execute_with(|| {
		assert_ok!(Chainlink::register_operator(RuntimeOrigin::signed(1)));
		assert_err!(
			Chainlink::initiate_request(
				RuntimeOrigin::signed(2),
				1,
				bounded_vec![],
				1,
				bounded_vec![],
				0,
				mock::pallet::Call::<Test>::callback { result: bounded_vec![] }.into()
			),
			<Error<Test>>::InsufficientFee
		);
	});

	new_test_ext().execute_with(|| {
		assert_err!(
			Chainlink::initiate_request(
				RuntimeOrigin::signed(2),
				1,
				bounded_vec![],
				1,
				bounded_vec![],
				1,
				mock::pallet::Call::<Test>::callback { result: bounded_vec![] }.into()
			),
			<Error<Test>>::UnknownOperator
		);
	});

	new_test_ext().execute_with(|| {
		assert_ok!(Chainlink::register_operator(RuntimeOrigin::signed(1)));
		assert_ok!(Chainlink::initiate_request(
			RuntimeOrigin::signed(2),
			1,
			bounded_vec![],
			1,
			bounded_vec![],
			2,
			mock::pallet::Call::<Test>::callback { result: bounded_vec![] }.into()
		));
		assert_err!(
			Chainlink::callback(RuntimeOrigin::signed(3), 0, 10.encode().try_into().unwrap()),
			<Error<Test>>::WrongOperator
		);
	});

	new_test_ext().execute_with(|| {
		assert_err!(
			Chainlink::callback(RuntimeOrigin::signed(1), 0, 10.encode().try_into().unwrap()),
			<Error<Test>>::UnknownRequest
		);
	});

	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(Chainlink::register_operator(RuntimeOrigin::signed(1)));
		assert_eq!(last_event(), Event::OperatorRegistered(1));

		let parameters = ("a", "b");
		let data: BoundedVec<u8, MaxRequestLen> = parameters.encode().try_into().unwrap();
		assert_ok!(Chainlink::initiate_request(
			RuntimeOrigin::signed(2),
			1,
			bounded_vec![],
			1,
			data.clone(),
			2,
			mock::pallet::Call::<Test>::callback { result: bounded_vec![] }.into()
		));
		assert_eq!(
			last_event(),
			Event::OracleRequest(1, bounded_vec![], 0, 2, 1, data.clone(), crate::CALLBACK_NAME, 2)
		);

		let r = <(Vec<u8>, Vec<u8>)>::decode(&mut &data[..]).unwrap().0;
		assert_eq!("a", std::str::from_utf8(&r).unwrap());

		let result = 10;
		assert_ok!(Chainlink::callback(
			RuntimeOrigin::signed(1),
			0,
			result.encode().try_into().unwrap()
		));
		assert_eq!(mock::pallet::Result::<Test>::get(), result);
	});
}

#[test]
pub fn on_finalize() {
	new_test_ext().execute_with(|| {
		assert_ok!(Chainlink::register_operator(RuntimeOrigin::signed(1)));
		assert_ok!(Chainlink::initiate_request(
			RuntimeOrigin::signed(2),
			1,
			bounded_vec![],
			1,
			bounded_vec![],
			2,
			mock::pallet::Call::<Test>::callback { result: bounded_vec![] }.into()
		));
		<Chainlink as OnFinalize<u64>>::on_finalize(20);
		// Request has been killed, too old
		assert_err!(
			Chainlink::callback(RuntimeOrigin::signed(1), 0, 10.encode().try_into().unwrap()),
			<Error<Test>>::UnknownRequest
		);
	});
}
