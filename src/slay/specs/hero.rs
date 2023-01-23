use enum_iterator::Sequence;

use crate::slay::abilities::heros::VictimDraws;
use crate::slay::modifiers::ModifierOrigin;
use crate::slay::modifiers::PlayerModifier;
use crate::slay::showdown::consequences::Condition;
use crate::slay::showdown::consequences::RollConsequence;
use crate::slay::showdown::consequences::RollConsequences;
use crate::slay::specification::HeroType;
use crate::slay::tasks::core::destroy::DestroyCardTask;
use crate::slay::tasks::core::destroy::DestroyModifiersDestination;
use crate::slay::tasks::core::destroy::DestroyTask;
use crate::slay::tasks::core::discard::Discard;
use crate::slay::tasks::core::draw::DrawTask;
use crate::slay::tasks::core::pull::PullFromTask;
use crate::slay::tasks::core::sacrifice::Sacrifice;
use crate::slay::tasks::core::steal::StealCardFromTask;
use crate::slay::tasks::core::steal::StealTask;
use crate::slay::tasks::heros::mimimeow::Mimimeow;
use crate::slay::tasks::heros::slippery_paws::SlipperyPaws;
use crate::slay::tasks::player_tasks::PlayerTask;
use crate::slay::tasks::task_params::TaskParamName;
use crate::slay::tasks::tasks::immediate::OfferPlayImmediately;
use crate::slay::tasks::tasks::immediate::PlayImmediatelyFilter;
use crate::slay::tasks::tasks::params::ChooseCardFromPlayerParameterTask;
use crate::slay::tasks::tasks::params::ChoosePlayerParameterTask;
use crate::slay::tasks::tasks::params::ClearParamsTask;
use crate::slay::tasks::tasks::receive_modifier::ReceiveModifier;
use crate::slay::tasks::tasks::return_modifiers::ReturnModifierTask;
use crate::slay::tasks::tasks::search_discard::SearchDiscard;
use crate::slay::tasks::tasks::trade_hands::TradeHands;

use super::cards::SlayCardSpec;

#[derive(Debug, Clone)]
pub struct HeroAbility {
	pub condition: Condition,
	pub ability: HeroAbilityType,
}

impl HeroAbility {
	pub fn to_consequences(&self) -> RollConsequences {
		RollConsequences {
			success: RollConsequence {
				condition: self.condition.to_owned(),
				tasks: self.ability.create_tasks(),
			},
			loss: None,
		}
	}
}

// Some renaming is appropriate...
// Call this HeroCard
#[derive(Clone, Debug, Sequence, PartialEq, Copy)]
pub enum HeroAbilityType {
	PlunderingPuma,
	SlipperyPaws,
	SmoothMimimeow,
	Meowzio,
	Shurikitty,
	KitNapper,
	SilentShadow,
	SlyPickings,
	HolyCurselifter,
	IronResolve,
	CalmingVoice,
	VibrantGlow,
	MightyBlade,
	RadiantHorn,
	GuidingLight,
	WiseShield,
	QiBear,
	PanChucks,
	HeavyBear,
	BadAxe,
	ToughTeddy,
	BearClaw,
	FuryKnuckle,
	BearyWise,
	Hook,
	Wildshot,
	SeriousGrey,
	WilyRed,
	QuickDraw,
	LookieRookie,
	Bullseye,
	SharpFox,
	FuzzyCheeks,
	Peanut,
	NappingNibbles,
	TipsyTootie,
	MellowDee,
	LuckBucky,
	DodgyDealer,
	GreedyCheeks,
	Fluffy,
	Wiggles,
	Spooky,
	Snowball,
	Buttons,
	BunBun,
	Hopper,
	Whiskers,
}

impl HeroAbilityType {
	pub fn label(&self) -> String {
		// Kind of backwards: that method should call this one...
		SlayCardSpec::HeroCard(*self).label()
	}

	pub fn to_consequences(&self) -> RollConsequences {
		if let Some(ability) = SlayCardSpec::HeroCard(*self)
			.get_card_spec_creation()
			.hero_ability
		{
			return ability.to_consequences();
		}
		{
			unreachable!();
		}
	}

	pub fn create_tasks(&self) -> Vec<Box<dyn PlayerTask>> {
		match self {
			HeroAbilityType::PlunderingPuma => vec![
				ChoosePlayerParameterTask::exclude_self(
					TaskParamName::PlayerToPullFrom,
					"Choose a player to pull from.",
				),
				PullFromTask::create(TaskParamName::PlayerToPullFrom),
				PullFromTask::create(TaskParamName::PlayerToPullFrom),
				VictimDraws::create(TaskParamName::PlayerToPullFrom),
				ClearParamsTask::create(),
			],
			HeroAbilityType::SlipperyPaws => vec![
				ChoosePlayerParameterTask::exclude_self(
					TaskParamName::SlipperyPawsVictim,
					"Choose a player to pull 2 cards from, you will have to discard one of them.",
				),
				PullFromTask::record_pulled(
					TaskParamName::SlipperyPawsVictim,
					Some(TaskParamName::SlipperyPawsVictimPulledCard1),
				),
				PullFromTask::record_pulled(
					TaskParamName::SlipperyPawsVictim,
					Some(TaskParamName::SlipperyPawsVictimPulledCard2),
				),
				SlipperyPaws::create(),
				ClearParamsTask::create(),
			],
			HeroAbilityType::SmoothMimimeow => vec![Mimimeow::create(), ClearParamsTask::create()],
			HeroAbilityType::Meowzio => vec![
				ChoosePlayerParameterTask::exclude_self(
					TaskParamName::MeowzioVictim,
					"Choose a player to steal and pull from.",
				),
				PullFromTask::create(TaskParamName::MeowzioVictim),
				ChooseCardFromPlayerParameterTask::from_party(
					TaskParamName::MeowzioVictim,
					TaskParamName::MeowzioCard,
					"Which hero card would you like to steal?",
				),
				StealCardFromTask::create(TaskParamName::MeowzioVictim, TaskParamName::MeowzioCard),
				ClearParamsTask::create(),
			],
			HeroAbilityType::Shurikitty => vec![
				ChoosePlayerParameterTask::exclude_self(
					TaskParamName::PlayerToDestroy,
					"to destroy a hero card (Shurikitty)",
				),
				ChooseCardFromPlayerParameterTask::from_party(
					TaskParamName::PlayerToDestroy,
					TaskParamName::CardToSteal,
					"Which hero card would you like to destroy?",
				),
				DestroyCardTask::create(
					TaskParamName::PlayerToDestroy,
					TaskParamName::CardToSteal,
					DestroyModifiersDestination::Myself,
				),
				ClearParamsTask::create(),
			],
			HeroAbilityType::KitNapper => vec![StealTask::create()],
			HeroAbilityType::SilentShadow => vec![
				ChoosePlayerParameterTask::exclude_self(
					TaskParamName::SilentShadowVictim,
					"Who's hand do you want to see?",
				),
				ChooseCardFromPlayerParameterTask::from_party(
					TaskParamName::SilentShadowVictim,
					TaskParamName::SilentShadowCard,
					"Which hero card would you like to take?",
				),
				// TODO
				ClearParamsTask::create(),
			],
			HeroAbilityType::SlyPickings => vec![
				ChoosePlayerParameterTask::exclude_self(
					TaskParamName::SlyPickinsVictim,
					"Sly Pickings: Who do you want to steal from?",
				),
				PullFromTask::record_pulled(
					TaskParamName::SlyPickinsVictim,
					Some(TaskParamName::SlyPickinsCard),
				),
				OfferPlayImmediately::create(
					TaskParamName::SlyPickinsCard,
					PlayImmediatelyFilter::IsMagic,
				),
				ClearParamsTask::create(),
			],
			HeroAbilityType::HolyCurselifter => vec![ReturnModifierTask::return_everyones()],
			HeroAbilityType::IronResolve => vec![ReceiveModifier::for_this_turn(
				PlayerModifier::NoCardsCanBeChallenged,
				ModifierOrigin::FromHeroAbility,
			)],
			HeroAbilityType::CalmingVoice => vec![ReceiveModifier::until_next_turn(
				PlayerModifier::NoCardsCanBeStolen,
				ModifierOrigin::FromHeroAbility,
			)],
			HeroAbilityType::VibrantGlow => vec![ReceiveModifier::for_this_turn(
				PlayerModifier::AddToAllRolls(5),
				ModifierOrigin::FromHeroAbility,
			)],
			HeroAbilityType::MightyBlade => vec![ReceiveModifier::until_next_turn(
				PlayerModifier::NoCardsCanBeDestroyed,
				ModifierOrigin::FromHeroAbility,
			)],
			HeroAbilityType::RadiantHorn => vec![SearchDiscard::for_modifiers()],
			HeroAbilityType::GuidingLight => vec![SearchDiscard::for_hero()],
			HeroAbilityType::WiseShield => vec![ReceiveModifier::for_this_turn(
				PlayerModifier::AddToAllRolls(3),
				ModifierOrigin::FromHeroAbility,
			)],
			HeroAbilityType::QiBear => vec![], ///////////////////////////////////////
			HeroAbilityType::PanChucks => vec![], ////////////////////////////////////
			HeroAbilityType::HeavyBear => vec![
				ChoosePlayerParameterTask::exclude_self(
					TaskParamName::HeavyBearVictim,
					"who has to discard 2 cards (Heavy bear)",
				),
				Discard::from_param(2, TaskParamName::HeavyBearVictim),
			],
			HeroAbilityType::BadAxe => vec![DestroyTask::create()],
			HeroAbilityType::ToughTeddy => vec![Discard::each_player_with(HeroType::Fighter)],
			HeroAbilityType::BearClaw => vec![], /////////////////////////////////////
			HeroAbilityType::FuryKnuckle => vec![], //////////////////////////////////
			HeroAbilityType::BearyWise => vec![], ////////////////////////////////////
			HeroAbilityType::Hook => vec![
				// There is similar logic in... immediately.rs
				DrawTask::create(1),
			],
			HeroAbilityType::Wildshot => vec![DrawTask::create(3), Discard::create(1)],
			HeroAbilityType::SeriousGrey => vec![DestroyTask::create(), DrawTask::create(1)],
			HeroAbilityType::WilyRed => vec![DrawTask::until(7)],
			HeroAbilityType::QuickDraw => vec![], ////////////////////////////////////
			HeroAbilityType::LookieRookie => vec![SearchDiscard::for_item()],
			HeroAbilityType::Bullseye => vec![], //////////////////////////////////////
			HeroAbilityType::SharpFox => vec![], //////////////////////////////////////
			HeroAbilityType::FuzzyCheeks => vec![], //////////////////////////////////
			HeroAbilityType::Peanut => vec![DrawTask::create(2)],
			HeroAbilityType::NappingNibbles => vec![/* This one actually is empty. */],
			HeroAbilityType::TipsyTootie => vec![], ///////////////////////////////////
			HeroAbilityType::MellowDee => vec![DrawTask::into_param(TaskParamName::MellowDeeVictim)],
			HeroAbilityType::LuckBucky => vec![], /////////////////////////////////////
			HeroAbilityType::DodgyDealer => vec![
				ChoosePlayerParameterTask::exclude_self(
					TaskParamName::DodgyDealerVictim,
					"Dodger Dealer: Who do you want to trade hands with?",
				),
				TradeHands::create(TaskParamName::DodgyDealerVictim),
				ClearParamsTask::create(),
			],
			HeroAbilityType::GreedyCheeks => vec![], //////////////////////////////////
			HeroAbilityType::Fluffy => vec![DestroyTask::create(), DestroyTask::create()],
			HeroAbilityType::Wiggles => vec![], ///////////////////////////////////////
			HeroAbilityType::Spooky => vec![],  ////////////////////////////////////////
			HeroAbilityType::Snowball => vec![], //////////////////////////////////////
			HeroAbilityType::Buttons => vec![], ///////////////////////////////////////
			HeroAbilityType::BunBun => vec![SearchDiscard::for_magic()],
			HeroAbilityType::Hopper => vec![
				ChoosePlayerParameterTask::exclude_self(
					TaskParamName::HopperVictim,
					"Hopper: Who do you want to sacrifice",
				),
				Sacrifice::from_param(TaskParamName::HopperVictim),
				ClearParamsTask::create(),
			],
			HeroAbilityType::Whiskers => vec![StealTask::create(), DestroyTask::create()],
		}
	}
}
